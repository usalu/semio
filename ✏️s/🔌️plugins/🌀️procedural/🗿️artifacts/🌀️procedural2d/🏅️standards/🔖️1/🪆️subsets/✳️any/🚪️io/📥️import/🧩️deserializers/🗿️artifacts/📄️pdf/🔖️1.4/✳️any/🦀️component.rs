//! procedural2d <- pdf
use crate::artifacts::procedural2d::Procedural2dSnapshot;
use semio_s_plugin_stdio::artifacts::pdf::{PdfSnapshot, STDIO_PDF_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &PdfSnapshot) -> Result<Procedural2dSnapshot, store::TextError> {
    let _ = (STDIO_PDF_DOCUMENT_SCHEMA, from);
    Ok(Procedural2dSnapshot::default())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Procedural2dSnapshot, store::TextError> {
    let _ = bytes;
    Ok(Procedural2dSnapshot::default())
}
