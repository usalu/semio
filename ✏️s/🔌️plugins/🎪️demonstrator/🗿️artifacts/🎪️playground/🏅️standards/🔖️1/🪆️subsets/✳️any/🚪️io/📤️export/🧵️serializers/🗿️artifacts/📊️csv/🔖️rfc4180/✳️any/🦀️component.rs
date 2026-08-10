//! playground -> csv
use crate::artifacts::playground::schema::snapshot::PlaygroundSnapshot;
use semio_s_plugin_stdio::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &PlaygroundSnapshot) -> Result<CsvSnapshot, store::TextError> {
    let value = serde_json::to_value(snapshot).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let headers = value.get("headers").and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
    let rows = value.get("rows").and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
    Ok(CsvSnapshot { schema: STDIO_CSV_DOCUMENT_SCHEMA.into(), headers, rows })
}

pub fn serialize_bytes(snapshot: &PlaygroundSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<CsvSnapshot as store::DocumentPack>::encode_pack(&serialize(snapshot)?))
}
