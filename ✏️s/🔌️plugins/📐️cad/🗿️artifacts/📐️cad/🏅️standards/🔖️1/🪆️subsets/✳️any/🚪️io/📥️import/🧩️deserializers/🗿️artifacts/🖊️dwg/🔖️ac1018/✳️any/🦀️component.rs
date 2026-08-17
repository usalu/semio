//! Deserialize cad via stdio.dwg.

use crate::artifacts::cad::CadSnapshot;
use semio_s_plugin_stdio::artifacts::dwg::{DwgSnapshot, STDIO_DWG_DOCUMENT_SCHEMA};

//#region Deserialize
pub fn register() {}

pub fn deserialize(_from: &DwgSnapshot) -> Result<CadSnapshot, store::TextError> {
    let _ = STDIO_DWG_DOCUMENT_SCHEMA;
    Ok(crate::artifacts::cad::empty_cad_snapshot())
}

pub fn deserialize_text(text: &str) -> Result<CadSnapshot, store::TextError> {
    <CadSnapshot as store::ArtifactDsl>::parse_dsl(text)
}
//#endregion Deserialize
