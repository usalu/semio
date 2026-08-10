//! Deserialize writer via stdio.docx.
use crate::artifacts::writer::{WriterSnapshot, WRITER_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::docx::{DocxSnapshot, STDIO_DOCX_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &DocxSnapshot) -> Result<WriterSnapshot, store::TextError> {
    let _ = STDIO_DOCX_DOCUMENT_SCHEMA;
    // 📰 Real paragraph/run model, not a raw-XML grep: each paragraph's runs are concatenated,
    // paragraphs joined by newlines — the honest text projection of the typed docx document.
    let body = from
        .document
        .paragraphs
        .iter()
        .map(|p| p.runs.iter().map(|r| r.text.as_str()).collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");
    Ok(WriterSnapshot {
        schema: WRITER_DOCUMENT_SCHEMA.into(),
        id: "docx-import".into(),
        language_id: "plain".into(),
        uri: "writer://docx-import".into(),
        text: body,
    })
}
