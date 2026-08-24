//! Deserialize cad via stdio.ifc.
//!
//! 🧹️ Ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W5a: the
//! binary `deserialize()` this file used to carry reinterpreted `IFCCARTESIANPOINT` values as the
//! CAD document's own opaque `ArtifactPack` bytes (fabricated, and had zero callers — `CadComposer`
//! only ever calls `deserialize_text` below) — deleted outright, matching the mirror export leaf.
use crate::artifacts::cad::CadSnapshot;

//#region Deserialize
pub fn register() {}

pub fn deserialize_text(text: &str) -> Result<CadSnapshot, store::TextError> {
    <CadSnapshot as store::ArtifactDsl>::parse_dsl(text)
}
//#endregion Deserialize
