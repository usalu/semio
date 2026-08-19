//! Serialize flow to stdio.json.
use crate::artifacts::flow::FlowSnapshot;
use semio_s_plugin_stdio::artifacts::json::JsonSnapshot;

pub async fn register() {}

pub async fn serialize(from: &FlowSnapshot) -> Result<JsonSnapshot, store::PackError> {
    let value = serde_json::to_value(from).map_err(|e| store::PackError::Schema(e.to_string()))?;
    Ok(JsonSnapshot::from_value(value))
}

pub async fn serialize_text(from: &FlowSnapshot) -> Result<String, store::PackError> {
    Ok(<FlowSnapshot as store::ArtifactDsl>::print_dsl(from))
}
