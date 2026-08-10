//! Serialize layout to stdio.pdf.
use crate::artifacts::layout::LayoutSnapshot;
use semio_s_plugin_stdio::artifacts::pdf::schema::snapshot::PageDoc;
use semio_s_plugin_stdio::artifacts::pdf::{PdfSnapshot, STDIO_PDF_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(from: &LayoutSnapshot) -> Result<PdfSnapshot, store::PackError> {
    Ok(PdfSnapshot {
        schema: STDIO_PDF_DOCUMENT_SCHEMA.into(),
        page: PageDoc { width: 612.0, height: 792.0, text: <LayoutSnapshot as store::DocumentDsl>::print_dsl(from) },
    })
}
