//! Serialize mathematical to stdio.csv.
use crate::artifacts::mathematical::MathematicalSnapshot;
use semio_s_plugin_stdio::artifacts::csv::CsvSnapshot;

pub fn register() {}

pub fn serialize(from: &MathematicalSnapshot) -> Result<CsvSnapshot, store::PackError> {
    serde_json::from_value(serde_json::to_value(from).map_err(|e| store::PackError::Schema(e.to_string()))?)
        .map_err(|e| store::PackError::Schema(e.to_string()))
}
