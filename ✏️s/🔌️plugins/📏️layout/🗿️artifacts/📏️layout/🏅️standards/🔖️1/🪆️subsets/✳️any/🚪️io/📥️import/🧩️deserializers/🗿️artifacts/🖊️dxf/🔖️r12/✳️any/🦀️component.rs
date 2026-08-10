//! Deserialize layout via stdio.dxf.
use crate::artifacts::layout::LayoutSnapshot;
use semio_s_plugin_stdio::artifacts::dxf::{DxfSnapshot, STDIO_DXF_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &DxfSnapshot) -> Result<LayoutSnapshot, store::TextError> {
    let _ = STDIO_DXF_DOCUMENT_SCHEMA;
    let text = serde_json::to_string(from).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    <LayoutSnapshot as store::DocumentDsl>::parse_dsl(&text)
}

pub fn deserialize_text(text: &str) -> Result<LayoutSnapshot, store::TextError> {
    <LayoutSnapshot as store::DocumentDsl>::parse_dsl(text)
}
