//! Deserialize flow via stdio.json.
use crate::FlowSnapshot;
use semio_s_artifact_stdio_json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<FlowSnapshot, semio_framework_diagnostic::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let dsl_value: semio_framework_value::DslValue = from.to_serde_value().into();
    semio_framework_value::FromValue::from_value(dsl_value).map_err(|e| semio_framework_diagnostic::TextError::new(e.kind, format!("flow<-json: {e}"), semio_framework_diagnostic::TextSpan::at(1, 1)))
}

pub fn deserialize_text(text: &str) -> Result<FlowSnapshot, semio_framework_diagnostic::TextError> {
    <FlowSnapshot as store::ArtifactDsl>::parse_dsl(text)
}
