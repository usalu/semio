//! wires -> md
use crate::artifacts::wires::schema::snapshot::WiresSnapshot;
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &WiresSnapshot) -> Result<MdSnapshot, store::TextError> {
    Ok(MdSnapshot {
        schema: STDIO_MD_DOCUMENT_SCHEMA.into(),
        body: <WiresSnapshot as store::ArtifactDsl>::print_dsl(snapshot),
    })
}

pub fn serialize_bytes(snapshot: &WiresSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<MdSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}
