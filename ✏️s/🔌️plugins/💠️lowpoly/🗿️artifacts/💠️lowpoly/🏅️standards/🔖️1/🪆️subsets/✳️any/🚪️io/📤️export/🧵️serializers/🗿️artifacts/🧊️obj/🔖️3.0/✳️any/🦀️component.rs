//! lowpoly -> obj
use crate::artifacts::lowpoly::schema::snapshot::LowpolySnapshot;
use semio_s_plugin_stdio::artifacts::obj::{ObjSnapshot, STDIO_OBJ_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn serialize(snapshot: &LowpolySnapshot) -> Result<ObjSnapshot, store::TextError> {
    let _ = STDIO_OBJ_DOCUMENT_SCHEMA;
    let bytes = <LowpolySnapshot as store::ArtifactPack>::encode_pack(snapshot);
    <ObjSnapshot as store::ArtifactPack>::decode_pack(&bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}

pub async fn serialize_bytes(snapshot: &LowpolySnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<ObjSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}
