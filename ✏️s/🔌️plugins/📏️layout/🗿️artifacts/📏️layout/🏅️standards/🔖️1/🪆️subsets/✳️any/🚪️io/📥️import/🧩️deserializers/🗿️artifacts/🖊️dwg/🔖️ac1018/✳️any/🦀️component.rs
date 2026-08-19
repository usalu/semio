//! Deserialize layout via stdio.dwg.
use crate::artifacts::layout::LayoutSnapshot;
use semio_s_plugin_stdio::artifacts::dwg::schema::snapshot::{decode_dwg, encode_dwg};
use semio_s_plugin_stdio::artifacts::dwg::{dwg_from_bytes, DwgDrawing, DwgSnapshot};

pub async fn register() {}

/// 🩹️ 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M6-remaining Part A: `DwgSnapshot` no
/// longer carries a raw `bytes` field (see stdio's real `📸️snapshot/🦀️component.rs`) -- same stale
/// drift as the sibling export serializer in this directory's `📤️export` counterpart. `encode_dwg`
/// re-materializes real DWG bytes from the structured snapshot so the existing byte-oriented
/// `deserialize_bytes`/`dwg_from_bytes` structural-codec path below needs no change.
pub async fn deserialize(from: &DwgSnapshot) -> Result<LayoutSnapshot, store::TextError> {
    let bytes = encode_dwg(from).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    deserialize_bytes(&bytes)
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<LayoutSnapshot, store::TextError> {
    let _meta = decode_dwg(bytes).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;
    let drawing: DwgDrawing = dwg_from_bytes(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let value = crate::artifacts::layout::io::layout_document_json_from_dwg(&drawing).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;
    serde_json::from_value(value).map_err(|e| store::TextError::new(format!("layout<-dwg: {e}"), dsl::TextSpan::at(1, 1)))
}
