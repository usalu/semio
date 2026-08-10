//! Deserialize writer via stdio.pdf.
use crate::artifacts::writer::{WriterSnapshot, WRITER_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::pdf::{PdfSnapshot, STDIO_PDF_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &PdfSnapshot) -> Result<WriterSnapshot, store::TextError> {
    let _ = STDIO_PDF_DOCUMENT_SCHEMA;
    Ok(WriterSnapshot {
        schema: WRITER_DOCUMENT_SCHEMA.into(),
        id: "pdf-import".into(),
        language_id: "plain".into(),
        uri: "writer://pdf-import".into(),
        text: from.page.text.clone(),
    })
}
