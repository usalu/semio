//! 🧬️ En1992 diff schema — sparse field delta over the artifact.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the En1992 artifact.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1992")]
pub struct En1992Diff {
    #[state(persistent)] pub artifact: Option<Box<crate::artifacts::en1992::schema::En1992Artifact>>,
    #[state(persistent)] pub annex: Option<crate::document::AnnexChoice>,
    #[state(persistent)] pub m_ed_knm: Option<f64>,
    #[state(persistent)] pub v_ed_kn: Option<f64>,
    #[state(persistent)] pub f_ck: Option<f64>,
    #[state(persistent)] pub b_mm: Option<f64>,
    #[state(persistent)] pub d_mm: Option<f64>,
    #[state(persistent)] pub a_s_mm2: Option<f64>,
    #[state(persistent)] pub f_yk: Option<f64>,
    #[state(persistent)] pub rho_l: Option<f64>,
    #[state(persistent)] pub n_ed_kn: Option<f64>,
    #[state(persistent)] pub p_kn: Option<f64>,
    #[state(persistent)] pub a_c_mm2: Option<f64>,
    #[state(persistent)] pub use_fem: Option<bool>,
    #[state(persistent)] pub span_m: Option<f64>,
    #[state(persistent)] pub udl_kn_m: Option<f64>,
    #[state(persistent)] pub fire_rating: Option<crate::artifacts::en1992::part_1_2::FireRating>,
    #[state(persistent)] pub provided_axis_distance_mm: Option<f64>,
    #[state(persistent)] pub bridge_sigma_c_mpa: Option<f64>,
    #[state(persistent)] pub bridge_delta_sigma_s_mpa: Option<f64>,
    #[state(persistent)] pub tightness_class: Option<crate::artifacts::en1992::part_3::TightnessClass>,
    #[state(persistent)] pub hd_over_h: Option<f64>,
    #[state(persistent)] pub liquid_sigma_s_mpa: Option<f64>,
    #[state(persistent)] pub liquid_rho_p_eff: Option<f64>,
    #[state(persistent)] pub liquid_f_ct_eff_mpa: Option<f64>,
    #[state(persistent)] pub liquid_e_s_mpa: Option<f64>,
    #[state(persistent)] pub liquid_s_r_max_mm: Option<f64>,
    #[state(persistent)] pub anchor_h_ef_mm: Option<f64>,
    #[state(persistent)] pub anchor_cracked: Option<bool>,
    #[state(persistent)] pub anchor_f_uk_mpa: Option<f64>,
    #[state(persistent)] pub anchor_f_yk_mpa: Option<f64>,
    #[state(persistent)] pub anchor_a_s_mm2: Option<f64>,
    #[state(persistent)] pub anchor_d_mm: Option<f64>,
    #[state(persistent)] pub anchor_c1_mm: Option<f64>,
    #[state(persistent)] pub anchor_n_ed_kn: Option<f64>,
    #[state(persistent)] pub anchor_v_ed_kn: Option<f64>,
    #[state(shared_ui)] pub selected_check_index: Option<Option<u32>>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 List wrapper for optional vector diffs.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct En1992StringList { pub values: Vec<String> }
//#endregion 🔖️DeltaHelpers
