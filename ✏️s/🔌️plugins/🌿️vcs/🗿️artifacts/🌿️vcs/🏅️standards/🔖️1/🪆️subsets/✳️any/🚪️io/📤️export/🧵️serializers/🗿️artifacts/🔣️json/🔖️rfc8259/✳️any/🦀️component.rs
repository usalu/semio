//! Serialize vcs to stdio.json.
use crate::artifacts::vcs::VcsSnapshot;
use semio_s_plugin_stdio::artifacts::json::JsonSnapshot;

pub fn register() {}

pub fn serialize(from: &VcsSnapshot) -> Result<JsonSnapshot, store::PackError> {
    let value = serde_json::to_value(from).map_err(|e| store::PackError::Schema(e.to_string()))?;
    Ok(JsonSnapshot::from_value(value))
}

pub fn serialize_text(from: &VcsSnapshot) -> Result<String, store::PackError> {
    Ok(<VcsSnapshot as store::ArtifactDsl>::print_dsl(from))
}
