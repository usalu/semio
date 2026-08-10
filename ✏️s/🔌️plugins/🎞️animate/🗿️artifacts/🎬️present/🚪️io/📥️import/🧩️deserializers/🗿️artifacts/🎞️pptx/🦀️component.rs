//! present <- pptx
use crate::artifacts::present::PresentSnapshot;
use semio_s_plugin_stdio::artifacts::pptx::{PptxSnapshot, STDIO_PPTX_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &PptxSnapshot) -> Result<PresentSnapshot, store::TextError> {
    let _ = STDIO_PPTX_DOCUMENT_SCHEMA;
    let bytes = semio_s_plugin_stdio::artifacts::pptx::engine::encode_pptx(from)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;
    <PresentSnapshot as store::DocumentPack>::decode_pack(&bytes)
        .or_else(|_| <PresentSnapshot as store::DocumentDsl>::parse_dsl(&String::from_utf8_lossy(&bytes)))
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<PresentSnapshot, store::TextError> {
    deserialize(&semio_s_plugin_stdio::artifacts::pptx::engine::decode_pptx(bytes)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?)
}
