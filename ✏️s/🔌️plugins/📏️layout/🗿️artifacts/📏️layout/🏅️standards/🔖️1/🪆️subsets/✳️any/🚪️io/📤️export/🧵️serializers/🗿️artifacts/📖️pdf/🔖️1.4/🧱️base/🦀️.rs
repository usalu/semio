//! 📖️ Layout document text interchange through the PDF 1.4 base page collection.
use crate::LayoutSnapshot;
use semio_s_artifact_stdio_pdf::standards::v1_4::subsets::base::schema::snapshot::{PageDoc, PdfSnapshot};
use semio_s_artifact_stdio_pdf::STDIO_PDF_DOCUMENT_SCHEMA;

pub fn register() {}

pub fn serialize(from: &LayoutSnapshot) -> Result<PdfSnapshot, store::PackError> {
    Ok(PdfSnapshot { schema: STDIO_PDF_DOCUMENT_SCHEMA.into(), pages: vec![PageDoc { width: 612.0, height: 792.0, text: <LayoutSnapshot as store::ArtifactDsl>::print_dsl(from) }] })
}
