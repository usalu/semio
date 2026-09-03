//! gismap -> json
use crate::artifacts::gismap::GisMapSnapshot;
use dsl::ToValue;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &GisMapSnapshot) -> Result<JsonSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let raw = serde_json::Value::from(&snapshot.to_value());
    Ok(JsonSnapshot::from_value(raw))
}

pub fn serialize_bytes(snapshot: &GisMapSnapshot) -> Result<Vec<u8>, store::TextError> {
    let value = serialize(snapshot)?.to_serde_value();
    serde_json::to_vec_pretty(&value).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}
