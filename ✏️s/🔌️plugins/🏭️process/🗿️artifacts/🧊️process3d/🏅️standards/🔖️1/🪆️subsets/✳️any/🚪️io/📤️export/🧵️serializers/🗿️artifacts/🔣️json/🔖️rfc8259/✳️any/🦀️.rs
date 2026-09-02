//! process3d -> json
use crate::artifacts::process3d::Process3dSnapshot;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

/// 🔣️ The document's `ToValue` tree rendered once as a JSON value — the single source both the
/// snapshot and the byte export are built from, so they can never disagree.
fn json_value(snapshot: &Process3dSnapshot) -> serde_json::Value {
    serde_json::Value::from(&semio_framework_os_kernel::ToValue::to_value(snapshot))
}

pub fn serialize(snapshot: &Process3dSnapshot) -> Result<JsonSnapshot, store::TextError> {
    Ok(JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value: json_value(snapshot).into() })
}

pub fn serialize_bytes(snapshot: &Process3dSnapshot) -> Result<Vec<u8>, store::TextError> {
    serde_json::to_vec_pretty(&json_value(snapshot)).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}
