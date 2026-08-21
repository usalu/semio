//! process3d -> dwg
use crate::artifacts::process3d::schema::snapshot::Process3dSnapshot;
use semio_s_plugin_stdio::artifacts::dwg::{DwgSnapshot, STDIO_DWG_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn serialize(snapshot: &Process3dSnapshot) -> Result<DwgSnapshot, store::TextError> {
    let _ = STDIO_DWG_DOCUMENT_SCHEMA;
    let bytes = <Process3dSnapshot as store::ArtifactPack>::encode_pack(snapshot);
    <DwgSnapshot as store::ArtifactPack>::decode_pack(&bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}

pub async fn serialize_bytes(snapshot: &Process3dSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<DwgSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}
