//! Serialize writer to stdio.pdf (the frozen 1.4 `PageDoc` subset — see stdio's own `📄️pdf`
//! artifact-root doc comment: TWO independent `PdfSnapshot` types share the `stdio.pdf` id family,
//! 1.4's is this plain single-page `PageDoc` shape, never the canonical 1.7 object-model one).
use crate::artifacts::writer::{writer_text, WriterSnapshot};
use semio_s_plugin_stdio::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::{PageDoc, PdfSnapshot};
use semio_s_plugin_stdio::artifacts::pdf::STDIO_PDF_DOCUMENT_SCHEMA;

pub fn register() {}

pub fn serialize(from: &WriterSnapshot) -> Result<PdfSnapshot, store::PackError> {
    Ok(PdfSnapshot {
        schema: STDIO_PDF_DOCUMENT_SCHEMA.into(),
        page: PageDoc { width: 612.0, height: 792.0, text: writer_text(from) },
    })
}
