//! present -> pptx
use crate::artifacts::present::PresentSnapshot;
use semio_s_plugin_stdio::artifacts::pptx::{PptxSnapshot, STDIO_PPTX_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &PresentSnapshot) -> Result<PptxSnapshot, store::TextError> {
    let _ = STDIO_PPTX_DOCUMENT_SCHEMA;
    let value = serde_json::to_value(snapshot).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    serde_json::from_value(value).map_err(|e| store::TextError::new(format!("present->pptx: {e}"), dsl::TextSpan::at(1, 1)))
}

pub fn serialize_bytes(snapshot: &PresentSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<PptxSnapshot as store::DocumentPack>::encode_pack(&serialize(snapshot)?))
}
