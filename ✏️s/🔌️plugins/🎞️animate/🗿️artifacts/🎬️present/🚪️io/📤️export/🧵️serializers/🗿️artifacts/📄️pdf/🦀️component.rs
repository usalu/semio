//! present -> pdf
use crate::artifacts::present::PresentSnapshot;
use semio_s_plugin_stdio::artifacts::pdf::{PdfSnapshot, STDIO_PDF_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &PresentSnapshot) -> Result<PdfSnapshot, store::TextError> {
    let bytes = <PresentSnapshot as store::DocumentPack>::encode_pack(snapshot)
        .or_else(|_| Ok(<PresentSnapshot as store::DocumentDsl>::print_dsl(snapshot).into_bytes()))?;
    semio_s_plugin_stdio::artifacts::pdf::engine::decode_pdf(&bytes)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
}

pub fn serialize_bytes(snapshot: &PresentSnapshot) -> Result<Vec<u8>, store::TextError> {
    semio_s_plugin_stdio::artifacts::pdf::engine::encode_pdf(&serialize(snapshot)?)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
}
