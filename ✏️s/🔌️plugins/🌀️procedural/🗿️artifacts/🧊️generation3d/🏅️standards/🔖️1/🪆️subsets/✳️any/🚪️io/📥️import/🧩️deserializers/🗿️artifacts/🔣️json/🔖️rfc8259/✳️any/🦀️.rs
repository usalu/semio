//! 🔣️ Restores graph documents through the owning JSON grammar and first-party value protocol.
use crate::Generation3dSnapshot;
use crate::standards::v1::subsets::any::io::mesh_bridge::io_error;
use semio_s_artifact_stdio_json::schema::snapshot::{parse_json_text, JsonSnapshot};

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<Generation3dSnapshot, semio_framework_diagnostic::TextError> {
    <Generation3dSnapshot as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&from.to_pack_value()))
        .map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error.under("generation3d←json"), semio_framework_diagnostic::TextSpan::at(1, 1)))
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Generation3dSnapshot, semio_framework_diagnostic::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|error| io_error(format!("generation3d←json: invalid UTF-8: {error}")))?;
    let value = parse_json_text(text).map_err(|mut error| { error.message = format!("generation3d←json: {}", error.message); error })?;
    deserialize(&JsonSnapshot::from_value(value))
}
