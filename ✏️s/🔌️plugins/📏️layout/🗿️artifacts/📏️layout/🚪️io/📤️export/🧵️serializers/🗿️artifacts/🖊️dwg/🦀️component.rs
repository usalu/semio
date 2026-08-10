//! Serialize layout to stdio.dwg.
use crate::artifacts::layout::LayoutSnapshot;
use semio_s_plugin_stdio::artifacts::dwg::{DwgSnapshot, STDIO_DWG_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(from: &LayoutSnapshot) -> Result<DwgSnapshot, store::PackError> {
    let value = serde_json::to_value(from).map_err(|e| store::PackError::Schema(e.to_string()))?;
    let bytes = serde_json::to_vec(&value).map_err(|e| store::PackError::Schema(e.to_string()))?;
    Ok(DwgSnapshot {
        schema: STDIO_DWG_DOCUMENT_SCHEMA.into(),
        version: String::new(),
        bytes,
        section_names: Vec::new(),
    })
}
