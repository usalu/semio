//! Serialize mathematical to stdio.md.
use crate::artifacts::mathematical::MathematicalSnapshot;
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(from: &MathematicalSnapshot) -> Result<MdSnapshot, store::PackError> {
    Ok(MdSnapshot { schema: STDIO_MD_DOCUMENT_SCHEMA.into(), body: <MathematicalSnapshot as store::DocumentDsl>::print_dsl(from) })
}

pub fn serialize_text(from: &MathematicalSnapshot) -> Result<String, store::PackError> {
    Ok(<MathematicalSnapshot as store::DocumentDsl>::print_dsl(from))
}
