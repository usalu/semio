//! Serialize forms to stdio.json.
use crate::artifacts::forms::FormsSnapshot;
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::parse_json_text;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

/// 🌉 Bridges via json's own RFC8259 text codec (`JsonSnapshot::value` is `JsonValue`, json's
/// own key-order/lexeme-preserving model, not `serde_json::Value` -- see json's snapshot module).
pub fn register() {}

pub fn serialize(from: &FormsSnapshot) -> Result<JsonSnapshot, store::PackError> {
    let text = serde_json::to_string(from).map_err(|e| store::PackError::Schema(e.to_string()))?;
    let value = parse_json_text(&text).map_err(|e| store::PackError::Schema(e.to_string()))?;
    Ok(JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value })
}

pub fn serialize_text(from: &FormsSnapshot) -> Result<String, store::PackError> {
    Ok(<FormsSnapshot as store::ArtifactDsl>::print_dsl(from))
}
