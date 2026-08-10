//! Deserialize layout via stdio.dwg.
use crate::artifacts::layout::LayoutSnapshot;
use semio_framework::{dwg_from_bytes, DwgDrawing};
use semio_s_plugin_stdio::artifacts::dwg::schema::snapshot::decode_dwg;
use semio_s_plugin_stdio::artifacts::dwg::DwgSnapshot;

pub fn register() {}

pub fn deserialize(from: &DwgSnapshot) -> Result<LayoutSnapshot, store::TextError> {
    deserialize_bytes(&from.bytes)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<LayoutSnapshot, store::TextError> {
    let _meta = decode_dwg(bytes).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;
    let drawing: DwgDrawing = dwg_from_bytes(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let value = crate::artifacts::layout::engine::layout_document_json_from_dwg(&drawing).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;
    serde_json::from_value(value).map_err(|e| store::TextError::new(format!("layout<-dwg: {e}"), dsl::TextSpan::at(1, 1)))
}
