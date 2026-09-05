//! generation2d <- pdf
use crate::artifacts::generation2d::Generation2dSnapshot;
use semio_s_plugin_stdio::artifacts::pdf::{PdfSnapshot, STDIO_PDF_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &PdfSnapshot) -> Result<Generation2dSnapshot, store::TextError> {
    let _ = (STDIO_PDF_DOCUMENT_SCHEMA, from);
    Ok(Generation2dSnapshot::default())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Generation2dSnapshot, store::TextError> {
    let _ = bytes;
    Ok(Generation2dSnapshot::default())
}
