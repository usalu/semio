//! present -> md
use crate::artifacts::present::schema::snapshot::PresentSnapshot;
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &PresentSnapshot) -> Result<MdSnapshot, store::TextError> {
    Ok(MdSnapshot {
        schema: STDIO_MD_DOCUMENT_SCHEMA.into(),
        body: <PresentSnapshot as store::ArtifactDsl>::print_dsl(snapshot),
    })
}

pub fn serialize_bytes(snapshot: &PresentSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<MdSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}
