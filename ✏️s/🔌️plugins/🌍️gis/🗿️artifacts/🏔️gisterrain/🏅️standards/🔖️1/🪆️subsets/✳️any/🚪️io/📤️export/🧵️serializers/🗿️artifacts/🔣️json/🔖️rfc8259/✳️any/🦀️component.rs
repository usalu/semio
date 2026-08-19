//! gisterrain -> json
use crate::artifacts::gisterrain::GisTerrainSnapshot;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn serialize(snapshot: &GisTerrainSnapshot) -> Result<JsonSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let raw = serde_json::to_value(snapshot).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    Ok(JsonSnapshot::from_value(raw))
}

pub async fn serialize_bytes(snapshot: &GisTerrainSnapshot) -> Result<Vec<u8>, store::TextError> {
    let value = serialize(snapshot)?.to_serde_value();
    serde_json::to_vec_pretty(&value).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}
