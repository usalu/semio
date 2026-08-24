//! Deserialize cad via stdio.stl.
//!
//! 🧹️ Ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W5a: the
//! binary `deserialize()` this file used to carry was already broken (real `StlSnapshot` has no
//! `vertices` field — `schema`/`solid_name`/`triangles` only) and had zero callers (`CadComposer`
//! only ever calls `deserialize_text` below) — deleted outright, matching the mirror export leaf.
use crate::artifacts::cad::CadSnapshot;

//#region Deserialize
pub fn register() {}

pub fn deserialize_text(text: &str) -> Result<CadSnapshot, store::TextError> {
    <CadSnapshot as store::ArtifactDsl>::parse_dsl(text)
}
//#endregion Deserialize
