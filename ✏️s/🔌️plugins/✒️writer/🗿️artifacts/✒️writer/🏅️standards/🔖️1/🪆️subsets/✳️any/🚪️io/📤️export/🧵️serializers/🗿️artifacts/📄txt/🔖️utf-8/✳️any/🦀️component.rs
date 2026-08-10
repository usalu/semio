//! Serialize writer to stdio.txt.
use crate::artifacts::writer::WriterSnapshot;
use semio_s_plugin_stdio::artifacts::txt::{TxtSnapshot, STDIO_TXT_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(from: &WriterSnapshot) -> Result<TxtSnapshot, store::PackError> {
    Ok(TxtSnapshot { schema: STDIO_TXT_DOCUMENT_SCHEMA.into(), text: <WriterSnapshot as store::ArtifactDsl>::print_dsl(from) })
}

pub fn serialize_text(from: &WriterSnapshot) -> Result<String, store::PackError> {
    Ok(<WriterSnapshot as store::ArtifactDsl>::print_dsl(from))
}
