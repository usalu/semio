//! process3d <- ifc
use crate::artifacts::process3d::schema::snapshot::Process3dSnapshot;
use semio_s_plugin_stdio::artifacts::ifc::{IfcSnapshot, STDIO_IFC_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &IfcSnapshot) -> Result<Process3dSnapshot, store::TextError> {
    let _ = STDIO_IFC_DOCUMENT_SCHEMA;
    let bytes = <IfcSnapshot as store::DocumentPack>::encode_pack(from);
    deserialize_bytes(&bytes)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Process3dSnapshot, store::TextError> {
    <Process3dSnapshot as store::DocumentPack>::decode_pack(bytes).or_else(|_| {
        <Process3dSnapshot as store::DocumentDsl>::parse_dsl(&String::from_utf8_lossy(bytes))
    })
}
