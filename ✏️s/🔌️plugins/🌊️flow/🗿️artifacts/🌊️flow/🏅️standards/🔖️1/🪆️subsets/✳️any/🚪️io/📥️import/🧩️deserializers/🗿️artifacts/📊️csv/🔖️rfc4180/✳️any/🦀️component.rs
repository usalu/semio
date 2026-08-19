//! Deserialize flow via stdio.csv.
use crate::artifacts::flow::FlowSnapshot;
use semio_s_plugin_stdio::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn deserialize(from: &CsvSnapshot) -> Result<FlowSnapshot, store::TextError> {
    let _ = STDIO_CSV_DOCUMENT_SCHEMA;
    let value = serde_json::to_value(from).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    serde_json::from_value(value).map_err(|e| store::TextError::new(format!("flow<-csv: {e}"), dsl::TextSpan::at(1, 1)))
}
