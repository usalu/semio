//! 🧬️ En1994 snapshot schema — persistent fields only.

use schema::ArtifactSchema;
use crate::document::AnnexChoice;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "norm.en1994", layout = "lines")]
#[artifact_schema(id = "s.norm.en1994")]
pub struct En1994Snapshot {
    #[state(persistent)]
    pub annex: AnnexChoice,
    #[state(persistent)]
    pub m_ed_knm: f64,
    #[dsl(unit = "kN")]
    #[state(persistent)]
    pub v_ed_kn: f64,
    #[state(persistent)]
    pub m_pla: f64,
    #[state(persistent)]
    pub m_pl_rd: f64,
    #[state(persistent)]
    pub eta: f64,
    #[state(persistent)]
    pub v_l_rd: f64,
    #[dsl(unit = "mm")]
    #[state(persistent)]
    pub insulation_thickness_mm: f64,
    #[state(persistent)]
    pub fire_rating: String,
    #[state(persistent)]
    pub deck_type: String,
    #[dsl(unit = "MPa")]
    #[state(persistent)]
    pub delta_sigma_mpa: f64,
    #[state(persistent)]
    pub fatigue_detail: String,
    #[dsl(unit = "mm")]
    #[state(persistent)]
    pub d_mm: f64,
    #[dsl(unit = "mm")]
    #[state(persistent)]
    pub h_sc_mm: f64,
    #[dsl(unit = "MPa")]
    #[state(persistent)]
    pub f_ck_mpa: f64,
    #[dsl(unit = "MPa")]
    #[state(persistent)]
    pub f_u_mpa: f64,
    #[dsl(unit = "MPa")]
    #[state(persistent)]
    pub e_cm_mpa: f64,
    #[dsl(unit = "kN")]
    #[state(persistent)]
    pub v_ed_per_stud_kn: f64,
    #[dsl(unit = "m")]
    #[state(persistent)]
    pub span_m: f64,
    #[dsl(unit = "MPa")]
    #[state(persistent)]
    pub f_y_mpa: f64,
    #[state(persistent)]
    pub n_cycles_stud: f64,
    #[dsl(unit = "MPa")]
    #[state(persistent)]
    pub delta_tau_stud_mpa: f64,
}
//#region 🔖️HandcraftedArtifactCodecs
// 🧬️ Consolidated (W5a, ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT): the fifteen norm families' identical
// ArtifactDsl/ArtifactPack envelope-wrap glue now lives once, in `crate::document`'s
// `NormArtifactRecord`/`norm_{parse,print}_dsl`/`norm_{encode,decode}_pack` (see that
// region's doc comment in `📄️artifact/🦀️component.rs` for why it can't collapse further
// than this one macro call — Rust's orphan rule still needs a concrete per-type impl).
crate::impl_norm_artifact_record!(En1994Snapshot, extension = "en1994", envelope_id = "norm.en1994");
//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for En1994Snapshot {
    fn default() -> Self {
        Self {
            annex: AnnexChoice::De,
            m_ed_knm: 200.0,
            v_ed_kn: 120.0,
            m_pla: 80.0,
            m_pl_rd: 250.0,
            eta: 0.75,
            v_l_rd: 150.0,
            insulation_thickness_mm: 20.0,
            fire_rating: "r60".into(),
            deck_type: "trapezoidal".into(),
            delta_sigma_mpa: 65.0,
            fatigue_detail: "stud_welded".into(),
            d_mm: 19.0,
            h_sc_mm: 95.0,
            f_ck_mpa: 30.0,
            f_u_mpa: 450.0,
            e_cm_mpa: 33_000.0,
            v_ed_per_stud_kn: 40.0,
            span_m: 8.0,
            f_y_mpa: 355.0,
            n_cycles_stud: 2_000_000.0,
            delta_tau_stud_mpa: 40.0,
        }
    }
}
//#endregion 🔖️Snapshot
