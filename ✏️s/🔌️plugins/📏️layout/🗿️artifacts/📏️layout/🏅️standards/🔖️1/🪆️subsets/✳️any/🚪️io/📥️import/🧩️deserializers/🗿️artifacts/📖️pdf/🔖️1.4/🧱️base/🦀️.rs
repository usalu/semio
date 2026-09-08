//! 📖️ Layout document text interchange through the PDF 1.4 base page collection.
use crate::LayoutSnapshot;
use semio_s_artifact_stdio_pdf::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot;
use semio_s_artifact_stdio_pdf::STDIO_PDF_DOCUMENT_SCHEMA;

pub fn register() {}

pub fn deserialize(from: &PdfSnapshot) -> Result<LayoutSnapshot, store::TextError> {
    let _ = STDIO_PDF_DOCUMENT_SCHEMA;
    <LayoutSnapshot as store::ArtifactDsl>::parse_dsl(&from.pages.iter().map(|page| page.text.as_str()).collect::<Vec<_>>().join("\n"))
}
