//! lowpoly -> ply
use crate::artifacts::lowpoly::LowpolySnapshot;
use semio_s_plugin_stdio::artifacts::ply::{PlySnapshot, STDIO_PLY_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &LowpolySnapshot) -> Result<PlySnapshot, store::TextError> {
    let bytes = <LowpolySnapshot as store::DocumentPack>::encode_pack(snapshot)
        .or_else(|_| Ok(<LowpolySnapshot as store::DocumentDsl>::print_dsl(snapshot).into_bytes()))?;
    semio_s_plugin_stdio::artifacts::ply::engine::decode_ply(&bytes)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
}

pub fn serialize_bytes(snapshot: &LowpolySnapshot) -> Result<Vec<u8>, store::TextError> {
    semio_s_plugin_stdio::artifacts::ply::engine::encode_ply(&serialize(snapshot)?)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
}
