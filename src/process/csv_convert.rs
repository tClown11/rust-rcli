use std::fs;

use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};

use crate::opts::OutputFormat;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    pub name: String,
    pub position: String,
    #[serde(rename = "DOB")]
    pub dob: String,
    pub nationality: String,
    #[serde(rename = "Kit Number")]
    pub kit: u8,
}

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        // headers.iter() -> 使用 headers 的迭代器
        // record.iter() -> 使用 record 的迭代器
        // zip() -> 将两个迭代器合并为一个元组的迭代器 [(header, record), ..]
        // collect::<Value>() -> 将元组的迭代器转换为 JSON Value
        let json_value = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();

        ret.push(json_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        // 暂不支持 toml
        //OutputFormat::Toml => toml::to_string(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };

    let json = serde_json::to_string_pretty(&content)?;
    fs::write(output, json)?; // => ()
    Ok(())
}
