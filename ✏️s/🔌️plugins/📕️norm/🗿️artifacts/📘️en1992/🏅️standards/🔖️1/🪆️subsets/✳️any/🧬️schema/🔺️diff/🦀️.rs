//! 🧬️ En1992 diff schema — sparse field delta over the artifact.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the En1992 artifact.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1992")]
pub struct En1992Diff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifacts::en1992::schema::En1992Artifact>>,
    #[state(artifact)]
    pub annex: Option<crate::document::AnnexChoice>,
    #[state(artifact)]
    pub m_ed_knm: Option<f64>,
    #[state(artifact)]
    pub v_ed_kn: Option<f64>,
    #[state(artifact)]
    pub f_ck: Option<f64>,
    #[state(artifact)]
    pub b_mm: Option<f64>,
    #[state(artifact)]
    pub d_mm: Option<f64>,
    #[state(artifact)]
    pub a_s_mm2: Option<f64>,
    #[state(artifact)]
    pub f_yk: Option<f64>,
    #[state(artifact)]
    pub rho_l: Option<f64>,
    #[state(artifact)]
    pub n_ed_kn: Option<f64>,
    #[state(artifact)]
    pub p_kn: Option<f64>,
    #[state(artifact)]
    pub a_c_mm2: Option<f64>,
    #[state(artifact)]
    pub use_fem: Option<bool>,
    #[state(artifact)]
    pub span_m: Option<f64>,
    #[state(artifact)]
    pub udl_kn_m: Option<f64>,
    #[state(artifact)]
    pub fire_rating: Option<crate::artifacts::en1992::part_1_2::FireRating>,
    #[state(artifact)]
    pub provided_axis_distance_mm: Option<f64>,
    #[state(artifact)]
    pub bridge_sigma_c_mpa: Option<f64>,
    #[state(artifact)]
    pub bridge_delta_sigma_s_mpa: Option<f64>,
    #[state(artifact)]
    pub tightness_class: Option<crate::artifacts::en1992::part_3::TightnessClass>,
    #[state(artifact)]
    pub hd_over_h: Option<f64>,
    #[state(artifact)]
    pub liquid_sigma_s_mpa: Option<f64>,
    #[state(artifact)]
    pub liquid_rho_p_eff: Option<f64>,
    #[state(artifact)]
    pub liquid_f_ct_eff_mpa: Option<f64>,
    #[state(artifact)]
    pub liquid_e_s_mpa: Option<f64>,
    #[state(artifact)]
    pub liquid_s_r_max_mm: Option<f64>,
    #[state(artifact)]
    pub anchor_h_ef_mm: Option<f64>,
    #[state(artifact)]
    pub anchor_cracked: Option<bool>,
    #[state(artifact)]
    pub anchor_f_uk_mpa: Option<f64>,
    #[state(artifact)]
    pub anchor_f_yk_mpa: Option<f64>,
    #[state(artifact)]
    pub anchor_a_s_mm2: Option<f64>,
    #[state(artifact)]
    pub anchor_d_mm: Option<f64>,
    #[state(artifact)]
    pub anchor_c1_mm: Option<f64>,
    #[state(artifact)]
    pub anchor_n_ed_kn: Option<f64>,
    #[state(artifact)]
    pub anchor_v_ed_kn: Option<f64>,
    #[state(presence)]
    pub selected_check_index: Option<Option<u32>>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 List wrapper for optional vector diffs.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct En1992StringList {
    pub values: Vec<String>,
}
//#endregion 🔖️DeltaHelpers
