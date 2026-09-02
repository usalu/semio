//! Serialize flow to stdio.json.
use crate::artifacts::flow::FlowSnapshot;
use semio_s_plugin_stdio::artifacts::json::JsonSnapshot;

pub fn register() {}

pub fn serialize(from: &FlowSnapshot) -> Result<JsonSnapshot, store::PackError> {
    let value: serde_json::Value = dsl::ToValue::to_value(from).into();
    Ok(JsonSnapshot::from_value(value))
}

pub fn serialize_text(from: &FlowSnapshot) -> Result<String, store::PackError> {
    Ok(<FlowSnapshot as store::ArtifactDsl>::print_dsl(from))
}
