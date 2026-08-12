//! Serialize cad to stdio.json.
//!
//! 🧹️ Ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W5a: the
//! binary `serialize()` this file used to carry no longer compiled against the real `JsonSnapshot`
//! shape (`value` is stdio's own `JsonValue`, not `serde_json::Value`) and had zero callers
//! (`CadComposer` only ever calls `serialize_text` below) — deleted outright.
use crate::artifacts::cad::CadSnapshot;

//#region Serialize
pub fn register() {}

pub fn serialize_text(from: &CadSnapshot) -> Result<String, store::PackError> {
    Ok(<CadSnapshot as store::ArtifactDsl>::print_dsl(from))
}
//#endregion Serialize
