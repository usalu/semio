//! Deserialize cad via stdio.gltf.
//!
//! 🧹️ Ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W5a: the
//! binary `deserialize()` this file used to carry no longer compiled against the real
//! `GltfSnapshot` shape (`document` is stdio's own typed `GltfDocument`, not a bare
//! `serde_json::Value`) and had zero callers (`CadComposer` only ever calls `deserialize_text`
//! below) — deleted outright.
use crate::artifacts::cad::CadSnapshot;

//#region Deserialize
pub fn register() {}

pub fn deserialize_text(text: &str) -> Result<CadSnapshot, store::TextError> {
    <CadSnapshot as store::ArtifactDsl>::parse_dsl(text)
}
//#endregion Deserialize
