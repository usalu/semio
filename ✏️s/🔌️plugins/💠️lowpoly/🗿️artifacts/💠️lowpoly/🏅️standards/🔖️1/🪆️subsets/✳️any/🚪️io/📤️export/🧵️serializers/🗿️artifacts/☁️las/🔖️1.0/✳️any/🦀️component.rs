//! lowpoly -> las
use crate::artifacts::lowpoly::schema::snapshot::LowpolySnapshot;
use semio_s_plugin_stdio::artifacts::las::{LasSnapshot, STDIO_LAS_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &LowpolySnapshot) -> Result<LasSnapshot, store::TextError> {
    let _ = STDIO_LAS_DOCUMENT_SCHEMA;
    let bytes = <LowpolySnapshot as store::DocumentPack>::encode_pack(snapshot);
    <LasSnapshot as store::DocumentPack>::decode_pack(&bytes)
        .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}

pub fn serialize_bytes(snapshot: &LowpolySnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<LasSnapshot as store::DocumentPack>::encode_pack(&serialize(snapshot)?))
}
