//! Deserialize cad via stdio.glb.

use crate::artifacts::cad::CadSnapshot;
use crate::artifacts::cad::io::{cad_from_wire, pack_err_as_text};
use semio_s_plugin_stdio::artifacts::glb::{GlbSnapshot, STDIO_GLB_DOCUMENT_SCHEMA};

//#region Deserialize
pub fn register() {}

pub fn deserialize(from: &GlbSnapshot) -> Result<CadSnapshot, store::TextError> {
    let _ = STDIO_GLB_DOCUMENT_SCHEMA;
    cad_from_wire(&from.payload.bin).map_err(pack_err_as_text)
}

pub fn deserialize_text(text: &str) -> Result<CadSnapshot, store::TextError> {
    <CadSnapshot as store::DocumentDsl>::parse_dsl(text)
}
//#endregion Deserialize
