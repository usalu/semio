//! playground <- csv
use crate::artifacts::playground::schema::snapshot::PlaygroundSnapshot;
use semio_s_plugin_stdio::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &CsvSnapshot) -> Result<PlaygroundSnapshot, store::TextError> {
    let _ = STDIO_CSV_DOCUMENT_SCHEMA;
    let value = serde_json::json!({ "headers": from.headers, "rows": from.rows });
    serde_json::from_value(value).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<PlaygroundSnapshot, store::TextError> {
    deserialize(&<CsvSnapshot as store::DocumentPack>::decode_pack(bytes)?)
}
