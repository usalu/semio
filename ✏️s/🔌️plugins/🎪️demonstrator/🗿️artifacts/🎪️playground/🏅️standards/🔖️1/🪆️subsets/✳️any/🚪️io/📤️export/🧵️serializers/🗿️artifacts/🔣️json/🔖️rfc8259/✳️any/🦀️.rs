//! playground -> json
use crate::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot;
use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::io::text::snapshot::write_json_pretty;
use semio_s_artifact_stdio_json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

/// 🌉 Bridges via json's own RFC8259 text codec (`JsonSnapshot::value` is `JsonValue`, json's
/// own key-order/lexeme-preserving model, not `serde_json::Value` -- see json's snapshot module).
pub fn serialize(snapshot: &PlaygroundSnapshot) -> Result<JsonSnapshot, semio_framework_diagnostic::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let value = semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(snapshot));
    Ok(JsonSnapshot::from_value(value))
}

pub fn serialize_bytes(snapshot: &PlaygroundSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    Ok(write_json_pretty(&serialize(snapshot)?.value).into_bytes())
}
