//! Serialize forms to stdio.xlsx.
use crate::artifacts::forms::FormsSnapshot;
use semio_s_plugin_stdio::artifacts::xlsx::XlsxSnapshot;

pub fn register() {}

pub fn serialize(from: &FormsSnapshot) -> Result<XlsxSnapshot, store::PackError> {
    let value = serde_json::to_value(from).map_err(|e| store::PackError::Schema(e.to_string()))?;
    serde_json::from_value(value).map_err(|e| store::PackError::Schema(e.to_string()))
}
