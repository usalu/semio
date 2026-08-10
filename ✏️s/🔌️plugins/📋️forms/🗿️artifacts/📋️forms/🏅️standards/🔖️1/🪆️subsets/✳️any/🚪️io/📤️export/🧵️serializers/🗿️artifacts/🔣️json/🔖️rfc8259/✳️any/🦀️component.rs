//! Serialize forms to stdio.json.
use crate::artifacts::forms::FormsSnapshot;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(from: &FormsSnapshot) -> Result<JsonSnapshot, store::PackError> {
    let value = serde_json::to_value(from).map_err(|e| store::PackError::Schema(e.to_string()))?;
    Ok(JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value })
}

pub fn serialize_text(from: &FormsSnapshot) -> Result<String, store::PackError> {
    Ok(<FormsSnapshot as store::ArtifactDsl>::print_dsl(from))
}
