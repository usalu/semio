//! process3d <- json
use crate::Process3dSnapshot;
use crate::PROCESS_3D_SCHEMA;
use semio_s_artifact_stdio_json::JsonSnapshot;

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<Process3dSnapshot, semio_framework_diagnostic::TextError> {
    let _ = PROCESS_3D_SCHEMA;
    let out = <Process3dSnapshot as semio_framework_value::FromValue>::from_value(semio_framework_value::DslValue::from(&from.to_serde_value())).map_err(|e| semio_framework_diagnostic::TextError::new(e.kind, format!("process3d<-json: {e}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    Ok(out)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Process3dSnapshot, semio_framework_diagnostic::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    let value: serde_json::Value = serde_json::from_str(text).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    deserialize(&JsonSnapshot::from_value(value))
}
