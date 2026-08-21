//! Serialize flow to stdio.csv.
use crate::artifacts::flow::FlowSnapshot;
use semio_s_plugin_stdio::artifacts::csv::CsvSnapshot;

pub async fn register() {}

pub async fn serialize(from: &FlowSnapshot) -> Result<CsvSnapshot, store::PackError> {
    serde_json::from_value(serde_json::to_value(from).map_err(|e| store::PackError::Schema(e.to_string()))?).map_err(|e| store::PackError::Schema(e.to_string()))
}
