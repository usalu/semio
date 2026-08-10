//! fem2d -> md
use crate::artifacts::fem2d::Fem2dSnapshot;
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &Fem2dSnapshot) -> Result<MdSnapshot, store::TextError> {
    Ok(MdSnapshot {
        schema: STDIO_MD_DOCUMENT_SCHEMA.into(),
        body: <Fem2dSnapshot as store::DocumentDsl>::print_dsl(snapshot),
    })
}

pub fn serialize_bytes(snapshot: &Fem2dSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(serialize(snapshot)?.body.into_bytes())
}
