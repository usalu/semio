//! Deserialize writer via stdio.txt.
use crate::artifacts::writer::WriterSnapshot;
use semio_s_plugin_stdio::artifacts::txt::{TxtSnapshot, STDIO_TXT_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &TxtSnapshot) -> Result<WriterSnapshot, store::TextError> {
    let _ = STDIO_TXT_DOCUMENT_SCHEMA;
    <WriterSnapshot as store::ArtifactDsl>::parse_dsl(&from.text)
}

pub fn deserialize_text(text: &str) -> Result<WriterSnapshot, store::TextError> {
    <WriterSnapshot as store::ArtifactDsl>::parse_dsl(text)
}
