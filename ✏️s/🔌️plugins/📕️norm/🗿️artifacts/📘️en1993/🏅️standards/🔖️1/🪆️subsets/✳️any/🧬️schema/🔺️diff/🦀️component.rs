//! 🧬️ En1993 diff schema — sparse field delta over the artifact.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the En1993 artifact.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1993")]
pub struct En1993Diff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifacts::en1993::schema::En1993Artifact>>,
    #[state(artifact)]
    pub annex: Option<crate::document::AnnexChoice>,
    #[state(artifact)]
    pub n_ed_kn: Option<f64>,
    #[state(artifact)]
    pub m_ed_knm: Option<f64>,
    #[state(artifact)]
    pub v_ed_kn: Option<f64>,
    #[state(artifact)]
    pub a_mm2: Option<f64>,
    #[state(artifact)]
    pub a_v_mm2: Option<f64>,
    #[state(artifact)]
    pub w_pl_mm3: Option<f64>,
    #[state(artifact)]
    pub f_y_mpa: Option<f64>,
    #[state(artifact)]
    pub f_u_mpa: Option<f64>,
    #[state(artifact)]
    pub chi: Option<f64>,
    #[state(artifact)]
    pub a_net_mm2: Option<f64>,
    #[state(artifact)]
    pub tension_n_ed_kn: Option<f64>,
    #[state(artifact)]
    pub fire_thickness_mm: Option<f64>,
    #[state(artifact)]
    pub fire_rating: Option<String>,
    #[state(artifact)]
    pub fire_massivity: Option<f64>,
    #[state(artifact)]
    pub fire_mu_0: Option<f64>,
    #[state(artifact)]
    pub fire_design_temperature_c: Option<f64>,
    #[state(artifact)]
    pub cf_b_bar_mm: Option<f64>,
    #[state(artifact)]
    pub cf_t_mm: Option<f64>,
    #[state(artifact)]
    pub cf_k_sigma: Option<f64>,
    #[state(artifact)]
    pub cf_psi: Option<f64>,
    #[state(artifact)]
    pub cf_n_ed_kn: Option<f64>,
    #[state(artifact)]
    pub cf_gross_resistance_kn: Option<f64>,
    #[state(artifact)]
    pub stainless_m_ed_knm: Option<f64>,
    #[state(artifact)]
    pub stainless_w_pl_mm3: Option<f64>,
    #[state(artifact)]
    pub stainless_f_y_mpa: Option<f64>,
    #[state(artifact)]
    pub plated_lambda_p: Option<f64>,
    #[state(artifact)]
    pub plated_sigma_ed_mpa: Option<f64>,
    #[state(artifact)]
    pub silo_t_mm: Option<f64>,
    #[state(artifact)]
    pub silo_r_mm: Option<f64>,
    #[state(artifact)]
    pub shell_sigma_x_ed_mpa: Option<f64>,
    #[state(artifact)]
    pub silo_k: Option<f64>,
    #[state(artifact)]
    pub silo_gamma_kn_m3: Option<f64>,
    #[state(artifact)]
    pub silo_depth_m: Option<f64>,
    #[state(artifact)]
    pub bolt_f_ed_kn: Option<f64>,
    #[state(artifact)]
    pub bolt_n_bolts: Option<u32>,
    #[state(artifact)]
    pub bolt_a_s_mm2: Option<f64>,
    #[state(artifact)]
    pub bolt_e1_mm: Option<f64>,
    #[state(artifact)]
    pub bolt_e2_mm: Option<f64>,
    #[state(artifact)]
    pub bolt_d0_mm: Option<f64>,
    #[state(artifact)]
    pub bolt_d_mm: Option<f64>,
    #[state(artifact)]
    pub bolt_t_mm: Option<f64>,
    #[state(artifact)]
    pub bolt_f_u_mpa: Option<f64>,
    #[state(artifact)]
    pub bolt_f_ub_mpa: Option<f64>,
    #[state(artifact)]
    pub weld_a_mm: Option<f64>,
    #[state(artifact)]
    pub weld_l_mm: Option<f64>,
    #[state(artifact)]
    pub weld_f_u_mpa: Option<f64>,
    #[state(artifact)]
    pub weld_steel_grade: Option<String>,
    #[state(artifact)]
    pub weld_f_ed_kn: Option<f64>,
    #[state(artifact)]
    pub delta_sigma_mpa: Option<f64>,
    #[state(artifact)]
    pub fatigue_category: Option<u8>,
    #[state(artifact)]
    pub fatigue_method: Option<String>,
    #[state(artifact)]
    pub t10_steel_subgrade: Option<String>,
    #[state(artifact)]
    pub t10_actual_thickness_mm: Option<f64>,
    #[state(artifact)]
    pub t10_t_ed_c: Option<f64>,
    #[state(artifact)]
    pub tension_component_f_uk_kn: Option<f64>,
    #[state(artifact)]
    pub tension_component_f_k_kn: Option<f64>,
    #[state(artifact)]
    pub tension_component_n_ed_kn: Option<f64>,
    #[state(artifact)]
    pub hss_w_el_mm3: Option<f64>,
    #[state(artifact)]
    pub hss_f_y_mpa: Option<f64>,
    #[state(artifact)]
    pub hss_section_class: Option<u8>,
    #[state(artifact)]
    pub hss_m_ed_knm: Option<f64>,
    #[state(artifact)]
    pub bridge_lambda: Option<f64>,
    #[state(artifact)]
    pub bridge_phi_2: Option<f64>,
    #[state(artifact)]
    pub bridge_delta_sigma_p_mpa: Option<f64>,
    #[state(artifact)]
    pub tower_wind_factor: Option<f64>,
    #[state(artifact)]
    pub tower_n_ed_kn: Option<f64>,
    #[state(artifact)]
    pub pile_sigma_mpa: Option<f64>,
    #[state(artifact)]
    pub pile_k_red: Option<f64>,
    #[state(artifact)]
    pub pile_n_ed_kn: Option<f64>,
    #[state(artifact)]
    pub crane_f_z_ed_kn: Option<f64>,
    #[state(artifact)]
    pub crane_wheel_contact_length_mm: Option<f64>,
    #[state(artifact)]
    pub crane_dispersion_mm: Option<f64>,
    #[state(artifact)]
    pub crane_t_w_mm: Option<f64>,
    #[state(presence)]
    pub selected_check_index: Option<Option<u32>>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 List wrapper for optional vector diffs.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct En1993StringList {
    pub values: Vec<String>,
}
//#endregion 🔖️DeltaHelpers
