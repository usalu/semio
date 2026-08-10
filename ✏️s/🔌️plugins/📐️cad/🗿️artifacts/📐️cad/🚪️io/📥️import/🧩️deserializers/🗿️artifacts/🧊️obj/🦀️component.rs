//! Deserialize cad via stdio.obj.

use crate::artifacts::cad::CadSnapshot;
use crate::artifacts::cad::io::{cad_from_wire, pack_err_as_text};
use semio_s_plugin_stdio::artifacts::obj::{ObjSnapshot, STDIO_OBJ_DOCUMENT_SCHEMA};

//#region Deserialize
pub fn register() {}

pub fn deserialize(from: &ObjSnapshot) -> Result<CadSnapshot, store::TextError> {
    let _ = STDIO_OBJ_DOCUMENT_SCHEMA;
    let mut bytes = Vec::with_capacity(from.vertices.len() * 12);
    for v in &from.vertices {
        bytes.extend_from_slice(&v.x.to_le_bytes());
        bytes.extend_from_slice(&v.y.to_le_bytes());
        bytes.extend_from_slice(&v.z.to_le_bytes());
    }
    // trailing face count marker unused; trim padding zeros from last incomplete vertex if needed
    cad_from_wire(&bytes).map_err(pack_err_as_text)
}

pub fn deserialize_text(text: &str) -> Result<CadSnapshot, store::TextError> {
    <CadSnapshot as store::DocumentDsl>::parse_dsl(text)
}
//#endregion Deserialize
