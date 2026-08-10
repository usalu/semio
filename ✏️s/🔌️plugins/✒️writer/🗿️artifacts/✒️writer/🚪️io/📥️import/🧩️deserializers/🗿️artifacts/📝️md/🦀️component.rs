//! Deserialize writer via stdio.md.
use crate::artifacts::writer::WriterSnapshot;
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &MdSnapshot) -> Result<WriterSnapshot, store::TextError> {
    let _ = STDIO_MD_DOCUMENT_SCHEMA;
    <WriterSnapshot as store::DocumentDsl>::parse_dsl(&from.body)
}

pub fn deserialize_text(text: &str) -> Result<WriterSnapshot, store::TextError> {
    <WriterSnapshot as store::DocumentDsl>::parse_dsl(text)
}
