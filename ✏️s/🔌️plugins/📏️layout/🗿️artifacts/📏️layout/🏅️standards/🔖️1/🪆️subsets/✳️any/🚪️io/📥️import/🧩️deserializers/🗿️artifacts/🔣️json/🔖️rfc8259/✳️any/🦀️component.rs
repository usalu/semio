//! Deserialize layout via stdio.json.
//!
//! 🩹️ w5b-close fix (stdio_gap/foreign-lag, not svg/dwg-pattern scope — see the paired export
//! leaf's doc comment and w5b-close-report.md). Mirrors it with the reverse structural converter.
use crate::artifacts::layout::LayoutSnapshot;
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::{JsonSnapshot, JsonValue};
use semio_s_plugin_stdio::artifacts::json::STDIO_JSON_DOCUMENT_SCHEMA;
use std::str::FromStr;

pub async fn register() {}

/// 🔁️ Structural `JsonValue -> serde_json::Value` conversion (reverse of the export leaf's
/// converter — see this file's module doc comment).
async fn json_value_to_serde(value: &JsonValue) -> serde_json::Value {
    match value {
        JsonValue::Null => serde_json::Value::Null,
        JsonValue::Bool { value } => serde_json::Value::Bool(*value),
        JsonValue::Number { lexeme } => serde_json::Number::from_str(lexeme).map(serde_json::Value::Number).unwrap_or(serde_json::Value::Null),
        JsonValue::String { value } => serde_json::Value::String(value.clone()),
        JsonValue::Array { items } => serde_json::Value::Array(items.iter().map(json_value_to_serde).collect()),
        JsonValue::Object { members } => serde_json::Value::Object(members.iter().map(|member| (member.key.clone(), json_value_to_serde(&member.value))).collect()),
    }
}

pub async fn deserialize(from: &JsonSnapshot) -> Result<LayoutSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    serde_json::from_value(from.to_serde_value()).map_err(|e| store::TextError::new(format!("layout<-json: {e}"), dsl::TextSpan::at(1, 1)))
}

pub async fn deserialize_text(text: &str) -> Result<LayoutSnapshot, store::TextError> {
    <LayoutSnapshot as store::ArtifactDsl>::parse_dsl(text)
}
