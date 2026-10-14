//! Deserialize cad via stdio.json.
//!
//! 🧹️ Ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W5a: the
//! binary `deserialize()` this file used to carry no longer compiled against the real
//! `JsonSnapshot` shape (`value` is stdio's own `JsonValue`, not `serde_json::Value`) and had zero
//! callers (`CadComposer` only ever calls `deserialize_text` below) — deleted outright.
use semio_s_artifact_cad_cad_model::snapshot::CadSnapshot;

//#region Deserialize
pub fn register() {}

pub fn deserialize_text(text: &str) -> Result<CadSnapshot, semio_framework_diagnostic::TextError> {
    <CadSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)
}
//#endregion Deserialize
