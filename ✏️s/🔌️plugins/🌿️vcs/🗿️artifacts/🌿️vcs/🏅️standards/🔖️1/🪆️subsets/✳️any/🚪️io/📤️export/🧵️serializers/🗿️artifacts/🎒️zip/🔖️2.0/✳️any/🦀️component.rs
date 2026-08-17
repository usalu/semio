//! Serialize vcs to stdio.zip.
use crate::artifacts::vcs::VcsSnapshot;
use semio_s_plugin_stdio::artifacts::zip::ZipSnapshot;

pub fn register() {}

pub fn serialize(from: &VcsSnapshot) -> Result<ZipSnapshot, store::PackError> {
    let value = serde_json::to_value(from).map_err(|e| store::PackError::Schema(e.to_string()))?;
    serde_json::from_value(value).map_err(|e| store::PackError::Schema(e.to_string()))
}
