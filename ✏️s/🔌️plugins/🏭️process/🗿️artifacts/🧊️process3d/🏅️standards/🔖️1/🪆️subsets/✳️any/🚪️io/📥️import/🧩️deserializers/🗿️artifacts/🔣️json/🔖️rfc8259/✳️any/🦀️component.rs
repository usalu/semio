//! process3d <- json
use crate::artifacts::process3d::Process3dSnapshot;
use crate::artifacts::process3d::PROCESS_3D_SCHEMA;
use semio_s_plugin_stdio::artifacts::json::JsonSnapshot;

pub async fn register() {}

pub async fn deserialize(from: &JsonSnapshot) -> Result<Process3dSnapshot, store::TextError> {
    let _ = PROCESS_3D_SCHEMA;
    let out: Process3dSnapshot = serde_json::from_value(from.to_serde_value())
        .map_err(|e| store::TextError::new(format!("process3d<-json: {e}"), dsl::TextSpan::at(1, 1)))?;
    Ok(out)
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<Process3dSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let value: serde_json::Value = serde_json::from_str(text).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    deserialize(&JsonSnapshot::from_value(value))
}
