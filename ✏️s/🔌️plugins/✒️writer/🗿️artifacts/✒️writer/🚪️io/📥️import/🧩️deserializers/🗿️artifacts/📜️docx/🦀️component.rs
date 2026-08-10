//! Deserialize writer via stdio.docx.
use crate::artifacts::writer::{WriterSnapshot, WRITER_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::docx::{DocxSnapshot, STDIO_DOCX_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &DocxSnapshot) -> Result<WriterSnapshot, store::TextError> {
    let _ = STDIO_DOCX_DOCUMENT_SCHEMA;
    let body = from.entries.iter().filter(|e| e.name.ends_with(".xml") || e.name.contains("document"))
        .filter_map(|e| std::str::from_utf8(&e.data).ok()).collect::<Vec<_>>().join("\n");
    Ok(WriterSnapshot {
        schema: WRITER_DOCUMENT_SCHEMA.into(),
        id: "docx-import".into(),
        language_id: "plain".into(),
        uri: "writer://docx-import".into(),
        text: body,
    })
}
