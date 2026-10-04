//! Deserialize layout via stdio.dwg.
use crate::LayoutSnapshot;
use semio_s_artifact_stdio_dwg::schema::snapshot::{decode_dwg, encode_dwg};
use semio_s_artifact_stdio_dwg::{dwg_from_bytes, DwgDrawing, DwgSnapshot};

pub fn register() {}

/// 🩹️ 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M6-remaining Part A: `DwgSnapshot` no
/// longer carries a raw `bytes` field (see stdio's real `📸️snapshot/🦀️.rs`) -- same stale
/// drift as the sibling export serializer in this directory's `📤️export` counterpart. `encode_dwg`
/// re-materializes real DWG bytes from the structured snapshot so the existing byte-oriented
/// `deserialize_bytes`/`dwg_from_bytes` structural-codec path below needs no change.
pub fn deserialize(from: &DwgSnapshot) -> Result<LayoutSnapshot, semio_framework_diagnostic::TextError> {
    let bytes = encode_dwg(from).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,e.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    deserialize_bytes(&bytes)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<LayoutSnapshot, semio_framework_diagnostic::TextError> {
    let _meta = decode_dwg(bytes).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,e, semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    let drawing: DwgDrawing = dwg_from_bytes(bytes).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,e, semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    let value = crate::io::layout_document_json_from_dwg(&drawing).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,e, semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    <LayoutSnapshot as semio_framework_value::FromValue>::from_value(value).map_err(|e| semio_framework_diagnostic::TextError::from_value_error(e,semio_framework_diagnostic::TextSpan::at(1,1)))
}
