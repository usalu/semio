//! Serialize flow to stdio.md.
use crate::artifacts::flow::FlowSnapshot;
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(from: &FlowSnapshot) -> Result<MdSnapshot, store::PackError> {
    Ok(MdSnapshot { schema: STDIO_MD_DOCUMENT_SCHEMA.into(), body: <FlowSnapshot as store::DocumentDsl>::print_dsl(from) })
}

pub fn serialize_text(from: &FlowSnapshot) -> Result<String, store::PackError> {
    Ok(<FlowSnapshot as store::DocumentDsl>::print_dsl(from))
}
