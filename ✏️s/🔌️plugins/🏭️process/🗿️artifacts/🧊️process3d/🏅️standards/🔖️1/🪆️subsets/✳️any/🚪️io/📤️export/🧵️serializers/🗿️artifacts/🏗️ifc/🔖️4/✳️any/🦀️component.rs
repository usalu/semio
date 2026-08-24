//! process3d -> ifc
use crate::artifacts::process3d::schema::snapshot::Process3dSnapshot;
use semio_s_plugin_stdio::artifacts::ifc::{IfcSnapshot, STDIO_IFC_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &Process3dSnapshot) -> Result<IfcSnapshot, store::TextError> {
    let _ = STDIO_IFC_DOCUMENT_SCHEMA;
    let bytes = <Process3dSnapshot as store::ArtifactPack>::encode_pack(snapshot);
    <IfcSnapshot as store::ArtifactPack>::decode_pack(&bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}

pub fn serialize_bytes(snapshot: &Process3dSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<IfcSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}
