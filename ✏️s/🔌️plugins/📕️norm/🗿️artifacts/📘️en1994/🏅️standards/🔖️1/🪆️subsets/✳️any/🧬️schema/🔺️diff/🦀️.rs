//! 🧬️ En1994 diff schema — sparse field delta over the artifact.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the En1994 artifact.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1994")]
pub struct En1994Diff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifacts::en1994::schema::En1994Artifact>>,
    #[state(artifact)]
    pub annex: Option<crate::document::AnnexChoice>,
    #[state(artifact)]
    pub m_ed_knm: Option<f64>,
    #[state(artifact)]
    pub v_ed_kn: Option<f64>,
    #[state(artifact)]
    pub m_pla: Option<f64>,
    #[state(artifact)]
    pub m_pl_rd: Option<f64>,
    #[state(artifact)]
    pub eta: Option<f64>,
    #[state(artifact)]
    pub v_l_rd: Option<f64>,
    #[state(artifact)]
    pub insulation_thickness_mm: Option<f64>,
    #[state(artifact)]
    pub fire_rating: Option<String>,
    #[state(artifact)]
    pub deck_type: Option<String>,
    #[state(artifact)]
    pub delta_sigma_mpa: Option<f64>,
    #[state(artifact)]
    pub fatigue_detail: Option<String>,
    #[state(artifact)]
    pub d_mm: Option<f64>,
    #[state(artifact)]
    pub h_sc_mm: Option<f64>,
    #[state(artifact)]
    pub f_ck_mpa: Option<f64>,
    #[state(artifact)]
    pub f_u_mpa: Option<f64>,
    #[state(artifact)]
    pub e_cm_mpa: Option<f64>,
    #[state(artifact)]
    pub v_ed_per_stud_kn: Option<f64>,
    #[state(artifact)]
    pub span_m: Option<f64>,
    #[state(artifact)]
    pub f_y_mpa: Option<f64>,
    #[state(artifact)]
    pub n_cycles_stud: Option<f64>,
    #[state(artifact)]
    pub delta_tau_stud_mpa: Option<f64>,
    #[state(presence)]
    pub selected_check_index: Option<Option<u32>>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 List wrapper for optional vector diffs.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct En1994StringList {
    pub values: Vec<String>,
}
//#endregion 🔖️DeltaHelpers
