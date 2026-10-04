//! gismap <- json
use crate::GisMapSnapshot;
use semio_framework_value::FromValue;
use semio_s_artifact_stdio_json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<GisMapSnapshot, semio_framework_diagnostic::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let dsl_value = semio_framework_value::DslValue::from(&from.to_serde_value());
    GisMapSnapshot::from_value(dsl_value).map_err(|e| semio_framework_diagnostic::TextError::new(e.kind, format!("gismap<-json: {e}"), semio_framework_diagnostic::TextSpan::at(1, 1)))
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<GisMapSnapshot, semio_framework_diagnostic::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    let raw: serde_json::Value = serde_json::from_str(text).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    deserialize(&JsonSnapshot::from_value(raw))
}
