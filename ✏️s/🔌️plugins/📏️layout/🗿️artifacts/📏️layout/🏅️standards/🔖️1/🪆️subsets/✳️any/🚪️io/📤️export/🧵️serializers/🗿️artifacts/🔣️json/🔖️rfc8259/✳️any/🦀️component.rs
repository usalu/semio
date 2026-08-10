//! Serialize layout to stdio.json.
use crate::artifacts::layout::LayoutSnapshot;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(from: &LayoutSnapshot) -> Result<JsonSnapshot, store::PackError> {
    let value = serde_json::to_value(from).map_err(|e| store::PackError::Schema(e.to_string()))?;
    Ok(JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value })
}

pub fn serialize_text(from: &LayoutSnapshot) -> Result<String, store::PackError> {
    Ok(<LayoutSnapshot as store::ArtifactDsl>::print_dsl(from))
}
