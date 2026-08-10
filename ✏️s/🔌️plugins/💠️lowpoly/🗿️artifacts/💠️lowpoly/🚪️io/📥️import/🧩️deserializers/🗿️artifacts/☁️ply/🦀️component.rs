//! lowpoly <- ply
use crate::artifacts::lowpoly::LowpolySnapshot;
use semio_s_plugin_stdio::artifacts::ply::{PlySnapshot, STDIO_PLY_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &PlySnapshot) -> Result<LowpolySnapshot, store::TextError> {
    let _ = STDIO_PLY_DOCUMENT_SCHEMA;
    let bytes = semio_s_plugin_stdio::artifacts::ply::engine::encode_ply(from)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;
    <LowpolySnapshot as store::DocumentPack>::decode_pack(&bytes)
        .or_else(|_| <LowpolySnapshot as store::DocumentDsl>::parse_dsl(&String::from_utf8_lossy(&bytes)))
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<LowpolySnapshot, store::TextError> {
    deserialize(&semio_s_plugin_stdio::artifacts::ply::engine::decode_ply(bytes)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?)
}
