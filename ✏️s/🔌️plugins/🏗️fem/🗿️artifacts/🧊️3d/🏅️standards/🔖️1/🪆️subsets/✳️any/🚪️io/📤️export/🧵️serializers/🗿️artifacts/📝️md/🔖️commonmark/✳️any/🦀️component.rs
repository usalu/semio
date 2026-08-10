//! fem3d -> md
use crate::artifacts::fem3d::Fem3dSnapshot;
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &Fem3dSnapshot) -> Result<MdSnapshot, store::TextError> {
    Ok(MdSnapshot {
        schema: STDIO_MD_DOCUMENT_SCHEMA.into(),
        body: <Fem3dSnapshot as store::DocumentDsl>::print_dsl(snapshot),
    })
}

pub fn serialize_bytes(snapshot: &Fem3dSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(serialize(snapshot)?.body.into_bytes())
}
