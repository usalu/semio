//! Serialize writer to stdio.docx.
use crate::artifacts::writer::WriterSnapshot;
use semio_s_plugin_stdio::artifacts::docx::schema::snapshot::DocxParagraph;
use semio_s_plugin_stdio::artifacts::docx::DocxSnapshot;

pub fn register() {}

/// 📰 Builds a real, minimal-but-valid docx package: one paragraph per source line, via the
/// docx artifact's own typed builder — not a fabricated/renamed text file inside a zip.
pub fn serialize(from: &WriterSnapshot) -> Result<DocxSnapshot, store::PackError> {
    let paragraphs = from.text.split('\n').map(DocxParagraph::text).collect::<Vec<_>>();
    let document = semio_s_plugin_stdio::artifacts::docx::schema::snapshot::DocxDocument { paragraphs };
    Ok(semio_s_plugin_stdio::artifacts::docx::engine::build_minimal_docx(document))
}
