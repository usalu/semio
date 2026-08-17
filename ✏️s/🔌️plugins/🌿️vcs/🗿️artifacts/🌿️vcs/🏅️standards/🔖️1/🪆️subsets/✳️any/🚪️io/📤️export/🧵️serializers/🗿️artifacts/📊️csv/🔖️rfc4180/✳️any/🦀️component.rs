//! Serialize vcs to stdio.csv.
use crate::artifacts::vcs::VcsSnapshot;
use semio_s_plugin_stdio::artifacts::csv::CsvSnapshot;

pub fn register() {}

pub fn serialize(from: &VcsSnapshot) -> Result<CsvSnapshot, store::PackError> {
    serde_json::from_value(serde_json::to_value(from).map_err(|e| store::PackError::Schema(e.to_string()))?)
        .map_err(|e| store::PackError::Schema(e.to_string()))
}
