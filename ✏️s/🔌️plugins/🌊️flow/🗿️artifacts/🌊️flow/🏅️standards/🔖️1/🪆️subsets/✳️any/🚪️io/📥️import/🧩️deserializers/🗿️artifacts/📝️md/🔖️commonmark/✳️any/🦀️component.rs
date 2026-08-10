//! Deserialize flow via stdio.md.
use crate::artifacts::flow::FlowSnapshot;
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &MdSnapshot) -> Result<FlowSnapshot, store::TextError> {
    let _ = STDIO_MD_DOCUMENT_SCHEMA;
    <FlowSnapshot as store::DocumentDsl>::parse_dsl(&from.body)
}

pub fn deserialize_text(text: &str) -> Result<FlowSnapshot, store::TextError> {
    <FlowSnapshot as store::DocumentDsl>::parse_dsl(text)
}
