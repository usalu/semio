//! 🧬️ En1990 snapshot schema — persistent fields only.

use schema::ArtifactSchema;
use crate::document::AnnexChoice;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "norm.en1990", layout = "lines")]
#[artifact_schema(id = "s.norm.en1990")]
pub struct En1990Snapshot {
    #[state(persistent)]
    pub g_k: f64,
    #[dsl(table)]
    #[state(persistent)]
    pub q_k: Vec<En1990QkEntry>,
    #[dsl(unit = "kN")]
    #[state(persistent)]
    pub resistance_kn: f64,
    #[state(persistent)]
    pub consequence_class: u8,
    #[state(persistent)]
    pub annex: AnnexChoice,
    /// 🌍️ Seismic accidental action A_Ed [kN] combined per Eq. 6.12b; 0.0 disables the seismic situation.
    #[dsl(unit = "kN")]
    #[state(persistent)]
    pub seismic_a_ed_kn: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct En1990QkEntry {
    #[dsl(positional)]
    pub category: String,
    #[dsl(positional)]
    pub value: f64,
}

//#region 🔖️HandcraftedArtifactCodecs
// 🧬️ Consolidated (W5a, ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT): the fifteen norm families' identical
// ArtifactDsl/ArtifactPack envelope-wrap glue now lives once, in `crate::document`'s
// `NormArtifactRecord`/`norm_{parse,print}_dsl`/`norm_{encode,decode}_pack` (see that
// region's doc comment in `📄️artifact/🦀️component.rs` for why it can't collapse further
// than this one macro call — Rust's orphan rule still needs a concrete per-type impl).
crate::impl_norm_artifact_record!(En1990Snapshot, extension = "en1990", envelope_id = "norm.en1990");
//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for En1990Snapshot {
    fn default() -> Self {
        Self { g_k: 100.0, q_k: vec![En1990QkEntry { category: "office".into(), value: 50.0 }, En1990QkEntry { category: "wind".into(), value: 30.0 }], resistance_kn: 300.0, consequence_class: 2, annex: AnnexChoice::De, seismic_a_ed_kn: 40.0 }
    }
}
//#endregion 🔖️Snapshot
