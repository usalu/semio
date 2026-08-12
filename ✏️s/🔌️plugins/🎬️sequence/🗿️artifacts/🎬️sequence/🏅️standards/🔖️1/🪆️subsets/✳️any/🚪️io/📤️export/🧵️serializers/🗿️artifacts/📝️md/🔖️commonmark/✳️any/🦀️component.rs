//! sequence -> md
use crate::artifacts::sequence::schema::snapshot::SequenceSnapshot;
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &SequenceSnapshot) -> Result<MdSnapshot, store::TextError> {
    let _ = STDIO_MD_DOCUMENT_SCHEMA;
    Ok(MdSnapshot::from_text(&<SequenceSnapshot as store::ArtifactDsl>::print_dsl(snapshot)))
}

pub fn serialize_bytes(snapshot: &SequenceSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<MdSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}
