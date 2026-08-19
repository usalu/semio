//! lowpoly <- stl
use crate::artifacts::lowpoly::schema::snapshot::LowpolySnapshot;
use semio_s_plugin_stdio::artifacts::stl::{StlSnapshot, STDIO_STL_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn deserialize(from: &StlSnapshot) -> Result<LowpolySnapshot, store::TextError> {
    let _ = STDIO_STL_DOCUMENT_SCHEMA;
    let bytes = <StlSnapshot as store::ArtifactPack>::encode_pack(from);
    deserialize_bytes(&bytes)
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<LowpolySnapshot, store::TextError> {
    <LowpolySnapshot as store::ArtifactPack>::decode_pack(bytes).or_else(|_| {
        <LowpolySnapshot as store::ArtifactDsl>::parse_dsl(&String::from_utf8_lossy(bytes))
    })
}
