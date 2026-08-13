//! 🧬️ Vdi3805 snapshot schema — artifact-lane fields only.

use crate::artifacts::vdi3805::{
    CatalogIndex, CharacteristicCurve, EditionId, EditionProfileChoice, ManufacturerCatalog, ManufacturerFile,
    ParametricGeometry, SecurityLimits,
};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Snapshot


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "norm.vdi3805", layout = "lines")]
#[artifact_schema(id = "s.norm.vdi3805")]
pub struct Vdi3805Snapshot {
    #[state(artifact)]
    pub manufacturer_file: ManufacturerFile,
    #[state(artifact)]
    pub catalog: ManufacturerCatalog,
    #[state(artifact)]
    pub edition_profile: BTreeMap<String, EditionProfileChoice>,
    #[state(artifact)]
    pub correction_as_of: EditionId,
    #[state(artifact)]
    pub strict_mode: bool,
    #[state(artifact)]
    pub index: CatalogIndex,
    #[state(artifact)]
    pub geometry: BTreeMap<String, ParametricGeometry>,
    #[state(artifact)]
    pub curves: BTreeMap<String, CharacteristicCurve>,
    #[state(artifact)]
    pub limits: SecurityLimits,
}
//#region 🔖️HandcraftedArtifactCodecs
// 🧬️ Consolidated (W5a, ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT): the fifteen norm families' identical
// ArtifactDsl/ArtifactPack envelope-wrap glue now lives once, in `crate::document`'s
// `NormArtifactRecord`/`norm_{parse,print}_dsl`/`norm_{encode,decode}_pack` (see that
// region's doc comment in `📄️artifact/🦀️component.rs` for why it can't collapse further
// than this one macro call — Rust's orphan rule still needs a concrete per-type impl).
crate::impl_norm_artifact_record!(Vdi3805Snapshot, extension = "vdi3805", envelope_id = "norm.vdi3805");
//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for Vdi3805Snapshot {
    fn default() -> Self {
        crate::artifacts::vdi3805::reference_fixture()
    }
}
//#endregion 🔖️Snapshot
