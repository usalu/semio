//! lowpoly <- json
use crate::LowpolySnapshot;
use crate::LOWPOLY_DOCUMENT_SCHEMA;
use semio_s_artifact_stdio_json::schema::snapshot::parse_json_text;
use semio_s_artifact_stdio_json::JsonSnapshot;

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<LowpolySnapshot, semio_framework_diagnostic::TextError> {
    let value: semio_framework_value::DslValue = from.to_serde_value().into();
    let mut out: LowpolySnapshot = semio_framework_value::FromValue::from_value(value).map_err(|e: semio_framework_value::ValueError| semio_framework_diagnostic::TextError::new(e.kind, format!("lowpoly<-json: {e}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    if out.schema.is_empty() {
        out.schema = LOWPOLY_DOCUMENT_SCHEMA.into();
    }
    Ok(out)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<LowpolySnapshot, semio_framework_diagnostic::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    let value = parse_json_text(text)?;
    deserialize(&JsonSnapshot::from_value(value))
}
