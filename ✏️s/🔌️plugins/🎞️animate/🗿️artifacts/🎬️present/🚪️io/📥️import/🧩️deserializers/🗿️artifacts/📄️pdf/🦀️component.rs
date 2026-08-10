//! present <- pdf
use crate::artifacts::present::PresentSnapshot;
use semio_s_plugin_stdio::artifacts::pdf::{PdfSnapshot, STDIO_PDF_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &PdfSnapshot) -> Result<PresentSnapshot, store::TextError> {
    let _ = STDIO_PDF_DOCUMENT_SCHEMA;
    let bytes = semio_s_plugin_stdio::artifacts::pdf::engine::encode_pdf(from)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;
    <PresentSnapshot as store::DocumentPack>::decode_pack(&bytes)
        .or_else(|_| <PresentSnapshot as store::DocumentDsl>::parse_dsl(&String::from_utf8_lossy(&bytes)))
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<PresentSnapshot, store::TextError> {
    deserialize(&semio_s_plugin_stdio::artifacts::pdf::engine::decode_pdf(bytes)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?)
}
