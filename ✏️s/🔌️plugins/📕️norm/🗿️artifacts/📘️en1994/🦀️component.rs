//! 🔗️ EN 1994 design of composite steel and concrete structures — document entities (constitutional: general).

use crate::core::AnnexChoice;
use serde::{Deserialize, Serialize};

//#region 🔖️Types
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "norm.en1994", layout = "lines")]
pub struct Document {
    pub annex: AnnexChoice,
    pub m_ed_knm: f64,
    #[dsl(unit = "kN")]
    pub v_ed_kn: f64,
    pub m_pla: f64,
    pub m_pl_rd: f64,
    pub eta: f64,
    pub v_l_rd: f64,
    #[dsl(unit = "mm")]
    pub insulation_thickness_mm: f64,
    pub fire_rating: String,
    pub deck_type: String,
    #[dsl(unit = "MPa")]
    pub delta_sigma_mpa: f64,
    pub fatigue_detail: String,
    #[dsl(unit = "mm")]
    pub d_mm: f64,
    #[dsl(unit = "mm")]
    pub h_sc_mm: f64,
    #[dsl(unit = "MPa")]
    pub f_ck_mpa: f64,
    #[dsl(unit = "MPa")]
    pub f_u_mpa: f64,
    #[dsl(unit = "MPa")]
    pub e_cm_mpa: f64,
    #[dsl(unit = "kN")]
    pub v_ed_per_stud_kn: f64,
    #[dsl(unit = "m")]
    pub span_m: f64,
    #[dsl(unit = "MPa")]
    pub f_y_mpa: f64,
    pub n_cycles_stud: f64,
    #[dsl(unit = "MPa")]
    pub delta_tau_stud_mpa: f64,
}

impl Default for Document {
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
//#endregion 🔖️Types

//#region 🔖️ArtifactKind
/// 🗿️ The computed-compliance artifact this standard publishes on its app's `report:out` port —
/// lifted out of the pre-migration manifest's inline `.artifact_kind(ArtifactKindSpec { .. })` so the
/// artifact node, not the app, owns its own kind declaration.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    crate::core::app::artifact_kind_spec("en1994", "EN 1994")
}
//#endregion 🔖️ArtifactKind
