//! 🧬️ Iso16757 snapshot schema — persistent fields only.

use crate::artifacts::iso16757::{part_1, part_2, part_4, part_5, CatalogueValue};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Snapshot


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "norm.iso16757", layout = "lines")]
#[artifact_schema(id = "s.norm.iso16757")]
pub struct Iso16757Snapshot {
    #[state(artifact)]
    pub catalogue: part_1::Catalogue,
    #[state(artifact)]
    pub dictionary: part_4::Dictionary,
    #[state(artifact)]
    pub geometry: part_2::GeometryCatalogue,
    #[state(artifact)]
    pub selection: part_1::SelectionRequest,
    #[state(artifact)]
    pub part_number_rule: part_5::PartNumberRule,
    #[state(artifact)]
    pub part_number_inputs: BTreeMap<String, CatalogueValue>,
    #[state(artifact)]
    pub script_limits: part_5::ScriptLimits,
    #[state(artifact)]
    pub exchange_process: part_5::ExchangeProcess,
}
//#region 🔖️HandcraftedArtifactCodecs
// 🧬️ Consolidated (W5a, ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT): the fifteen norm families' identical
// ArtifactDsl/ArtifactPack envelope-wrap glue now lives once, in `crate::document`'s
// `NormArtifactRecord`/`norm_{parse,print}_dsl`/`norm_{encode,decode}_pack` (see that
// region's doc comment in `📄️artifact/🦀️component.rs` for why it can't collapse further
// than this one macro call — Rust's orphan rule still needs a concrete per-type impl).
crate::impl_norm_artifact_record!(Iso16757Snapshot, extension = "iso16757", envelope_id = "norm.iso16757");
//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for Iso16757Snapshot {
    fn default() -> Self {
        Self::reference_fixture()
    }
}
//#endregion 🔖️Snapshot
