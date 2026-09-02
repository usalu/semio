//! Serialize flow to stdio.csv.
use crate::artifacts::flow::FlowSnapshot;
use semio_s_plugin_stdio::artifacts::csv::CsvSnapshot;

pub fn register() {}

pub fn serialize(from: &FlowSnapshot) -> Result<CsvSnapshot, store::PackError> {
    let value: serde_json::Value = dsl::ToValue::to_value(from).into();
    serde_json::from_value(value).map_err(|e| store::PackError::Schema(e.to_string()))
}
