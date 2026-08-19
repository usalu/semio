//! gismap -> json
use crate::artifacts::gismap::GisMapSnapshot;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn serialize(snapshot: &GisMapSnapshot) -> Result<JsonSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let raw = serde_json::to_value(snapshot).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    Ok(JsonSnapshot::from_value(raw))
}

pub async fn serialize_bytes(snapshot: &GisMapSnapshot) -> Result<Vec<u8>, store::TextError> {
    let value = serialize(snapshot)?.to_serde_value();
    serde_json::to_vec_pretty(&value).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}
