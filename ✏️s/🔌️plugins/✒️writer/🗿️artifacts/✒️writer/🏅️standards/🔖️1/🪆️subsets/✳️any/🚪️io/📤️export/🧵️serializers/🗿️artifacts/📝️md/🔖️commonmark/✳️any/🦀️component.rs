//! Serialize writer to stdio.md.
use crate::artifacts::writer::WriterSnapshot;
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(from: &WriterSnapshot) -> Result<MdSnapshot, store::PackError> {
    Ok(MdSnapshot { schema: STDIO_MD_DOCUMENT_SCHEMA.into(), body: <WriterSnapshot as store::DocumentDsl>::print_dsl(from) })
}

pub fn serialize_text(from: &WriterSnapshot) -> Result<String, store::PackError> {
    Ok(<WriterSnapshot as store::DocumentDsl>::print_dsl(from))
}
