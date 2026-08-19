//! process3d -> json
use crate::artifacts::process3d::Process3dSnapshot;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn serialize(snapshot: &Process3dSnapshot) -> Result<JsonSnapshot, store::TextError> {
    Ok(JsonSnapshot {
        schema: STDIO_JSON_DOCUMENT_SCHEMA.into(),
        value: serde_json::to_value(snapshot).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?.into(),
    })
}

pub async fn serialize_bytes(snapshot: &Process3dSnapshot) -> Result<Vec<u8>, store::TextError> {
    serde_json::to_vec_pretty(&serialize(snapshot)?.value).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}
