//! Serialize writer to stdio.docx.
use crate::artifacts::writer::WriterSnapshot;
use semio_s_plugin_stdio::artifacts::docx::schema::snapshot::DocxEntry;
use semio_s_plugin_stdio::artifacts::docx::{DocxSnapshot, STDIO_DOCX_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(from: &WriterSnapshot) -> Result<DocxSnapshot, store::PackError> {
    Ok(DocxSnapshot {
        schema: STDIO_DOCX_DOCUMENT_SCHEMA.into(),
        entries: vec![DocxEntry { name: "document.txt".into(), data: from.text.clone().into_bytes() }],
    })
}
