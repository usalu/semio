//! Deserialize layout via stdio.pdf.
//!
//! 🔀️ Same version-pinned import fix as the sibling export serializer — see that file's doc comment.
use crate::artifacts::layout::LayoutSnapshot;
use semio_s_plugin_stdio::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::PdfSnapshot;
use semio_s_plugin_stdio::artifacts::pdf::STDIO_PDF_DOCUMENT_SCHEMA;

pub fn register() {}

pub fn deserialize(from: &PdfSnapshot) -> Result<LayoutSnapshot, store::TextError> {
    let _ = STDIO_PDF_DOCUMENT_SCHEMA;
    <LayoutSnapshot as store::ArtifactDsl>::parse_dsl(&from.page.text)
}
