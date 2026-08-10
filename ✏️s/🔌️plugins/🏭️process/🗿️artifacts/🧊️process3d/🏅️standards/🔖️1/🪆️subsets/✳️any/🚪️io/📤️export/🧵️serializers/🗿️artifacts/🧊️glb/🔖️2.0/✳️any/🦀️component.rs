//! process3d -> glb
use crate::artifacts::process3d::schema::snapshot::Process3dSnapshot;
use semio_s_plugin_stdio::artifacts::glb::{GlbSnapshot, STDIO_GLB_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &Process3dSnapshot) -> Result<GlbSnapshot, store::TextError> {
    let _ = STDIO_GLB_DOCUMENT_SCHEMA;
    let bytes = <Process3dSnapshot as store::DocumentPack>::encode_pack(snapshot);
    <GlbSnapshot as store::DocumentPack>::decode_pack(&bytes)
        .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}

pub fn serialize_bytes(snapshot: &Process3dSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<GlbSnapshot as store::DocumentPack>::encode_pack(&serialize(snapshot)?))
}
