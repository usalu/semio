//! Deserialize layout via stdio.pdf.
use crate::artifacts::layout::LayoutSnapshot;
use semio_s_plugin_stdio::artifacts::pdf::{PdfSnapshot, STDIO_PDF_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &PdfSnapshot) -> Result<LayoutSnapshot, store::TextError> {
    let _ = STDIO_PDF_DOCUMENT_SCHEMA;
    <LayoutSnapshot as store::DocumentDsl>::parse_dsl(&from.page.text)
}
