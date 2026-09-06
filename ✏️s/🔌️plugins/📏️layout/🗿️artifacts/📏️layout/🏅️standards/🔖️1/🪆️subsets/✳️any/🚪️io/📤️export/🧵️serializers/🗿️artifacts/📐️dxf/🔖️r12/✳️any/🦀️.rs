//! Serialize layout to stdio.dxf.
use crate::artifacts::layout::LayoutSnapshot;
use semio_s_plugin_stdio::artifacts::dxf::{DxfSnapshot, STDIO_DXF_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(from: &LayoutSnapshot) -> Result<DxfSnapshot, store::PackError> {
    let value = serde_json::to_value(from).map_err(|e| store::PackError::Schema(e.to_string()))?;
    serde_json::from_value(value).map_err(|e| store::PackError::Schema(e.to_string()))
}
