//! 🌍️ EN 1997 snapshot schema — artifact-lane fields only.

use crate::document::AnnexChoice;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted EN 1997 document snapshot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "norm.en1997", layout = "lines")]
#[artifact_schema(id = "s.norm.en1997")]
pub struct En1997Snapshot {
    #[state(artifact)]
    pub v_ed_kn: f64,
    #[state(artifact)]
    pub h_ed_kn: f64,
    #[state(artifact)]
    pub footing_area_m2: f64,
    #[state(artifact)]
    pub phi_deg: f64,
    #[state(artifact)]
    pub c_kpa: f64,
    #[state(artifact)]
    pub gamma_kn_m3: f64,
    #[state(artifact)]
    pub b_m: f64,
    #[state(artifact)]
    pub d_f_m: f64,
    #[state(artifact)]
    pub e_s_mpa: f64,
    #[state(artifact)]
    pub nu: f64,
    #[state(artifact)]
    pub design_approach: String,
    #[state(artifact)]
    pub annex: crate::document::AnnexChoice,
    #[state(artifact)]
    pub settlement_limit_mm: f64,
    #[state(artifact)]
    pub n_pile_ed_kn: f64,
    #[state(artifact)]
    pub alpha_s: f64,
    #[state(artifact)]
    pub pile_d_m: f64,
    #[state(artifact)]
    pub q_s_kpa: f64,
    #[state(artifact)]
    pub pile_l_m: f64,
    #[state(artifact)]
    pub q_b_kpa: f64,
    #[state(artifact)]
    pub pile_base_area_m2: f64,
    #[state(artifact)]
    pub pile_n_profiles: u32,
    #[state(artifact)]
    pub z_investigated_m: f64,
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
// 🧬️ Consolidated (W5a, ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT): the fifteen norm families' identical
// ArtifactDsl/ArtifactPack envelope-wrap glue now lives once, in `crate::document`'s
// `NormArtifactRecord`/`norm_{parse,print}_dsl`/`norm_{encode,decode}_pack` (see that
// region's doc comment in `📄️artifact/🦀️component.rs` for why it can't collapse further
// than this one macro call — Rust's orphan rule still needs a concrete per-type impl).
crate::impl_norm_artifact_record!(En1997Snapshot, extension = "en1997", envelope_id = "norm.en1997");
//#endregion 🔖️HandcraftedArtifactCodecs


impl Default for En1997Snapshot {
    fn default() -> Self {
        Self {
            v_ed_kn: 500.0,
            h_ed_kn: 80.0,
            footing_area_m2: 2.0,
            phi_deg: 30.0,
            c_kpa: 0.0,
            gamma_kn_m3: 18.0,
            b_m: 2.0,
            d_f_m: 1.5,
            e_s_mpa: 30_000.0,
            nu: 0.3,
            design_approach: "da1str".into(),
            annex: AnnexChoice::De,
            settlement_limit_mm: 25.0,
            n_pile_ed_kn: 800.0,
            alpha_s: 0.7,
            pile_d_m: 0.6,
            q_s_kpa: 80.0,
            pile_l_m: 12.0,
            q_b_kpa: 2500.0,
            pile_base_area_m2: 0.28,
            pile_n_profiles: 1,
            z_investigated_m: 8.0,
        }
    }
}
