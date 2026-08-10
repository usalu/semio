//! Serialize mathematical to stdio.json.
use crate::artifacts::mathematical::MathematicalSnapshot;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(from: &MathematicalSnapshot) -> Result<JsonSnapshot, store::PackError> {
    let value = serde_json::to_value(from).map_err(|e| store::PackError::Schema(e.to_string()))?;
    Ok(JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value })
}

pub fn serialize_text(from: &MathematicalSnapshot) -> Result<String, store::PackError> {
    Ok(<MathematicalSnapshot as store::DocumentDsl>::print_dsl(from))
}
