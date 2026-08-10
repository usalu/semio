//! Deserialize cad via stdio.json.

use crate::artifacts::cad::CadSnapshot;
use crate::artifacts::cad::io::{cad_from_wire, pack_err_as_text};
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

//#region Deserialize
pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<CadSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    serde_json::from_value(from.value.clone()).map_err(|e| store::TextError::new(format!("cad<-json: {e}"), dsl::TextSpan::at(1, 1)))
}

pub fn deserialize_text(text: &str) -> Result<CadSnapshot, store::TextError> {
    <CadSnapshot as store::ArtifactDsl>::parse_dsl(text)
}
//#endregion Deserialize
