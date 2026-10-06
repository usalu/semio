//! playground <- json
use crate::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot;
use crate::PLAYGROUND_DOCUMENT_SCHEMA;
use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::io::text::snapshot::parse_json_text;
use semio_s_artifact_stdio_json::JsonSnapshot;

pub fn deserialize(from: &JsonSnapshot) -> Result<PlaygroundSnapshot, semio_framework_diagnostic::TextError> {
    let dsl_value = semio_framework_pack_json::to_dsl_value(&from.to_pack_value());
    let mut out: PlaygroundSnapshot = semio_framework_value::FromValue::from_value(dsl_value).map_err(|e| semio_framework_diagnostic::TextError::new(e.kind, format!("playground<-json: {e}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    if out.schema.is_empty() {
        out.schema = PLAYGROUND_DOCUMENT_SCHEMA.into();
    }
    Ok(out)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<PlaygroundSnapshot, semio_framework_diagnostic::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    let value = parse_json_text(text)?;
    deserialize(&JsonSnapshot::from_value(value))
}
