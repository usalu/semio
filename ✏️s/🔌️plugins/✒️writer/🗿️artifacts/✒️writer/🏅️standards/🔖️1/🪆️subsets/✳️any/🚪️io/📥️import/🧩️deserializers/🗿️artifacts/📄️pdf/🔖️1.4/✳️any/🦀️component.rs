//! Deserialize writer via stdio.pdf (the frozen 1.4 `PageDoc` subset — see the sibling export
//! serializer's doc comment).
use crate::artifacts::writer::{writer_snapshot_with_text, WriterSnapshot, WRITER_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::PdfSnapshot;
use semio_s_plugin_stdio::artifacts::pdf::STDIO_PDF_DOCUMENT_SCHEMA;

pub fn register() {}

pub fn deserialize(from: &PdfSnapshot) -> Result<WriterSnapshot, store::TextError> {
    let _ = STDIO_PDF_DOCUMENT_SCHEMA;
    Ok(writer_snapshot_with_text(WRITER_DOCUMENT_SCHEMA, "pdf-import", "plain", "writer://pdf-import", &from.page.text))
}
