//! 🧬️ En1993 diff schema — sparse field delta over the artifact.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the En1993 artifact.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1993")]
pub struct En1993Diff {
    #[state(persistent)] pub artifact: Option<Box<crate::artifacts::en1993::schema::En1993Artifact>>,
    #[state(persistent)] pub annex: Option<crate::document::AnnexChoice>,
    #[state(persistent)] pub n_ed_kn: Option<f64>,
    #[state(persistent)] pub m_ed_knm: Option<f64>,
    #[state(persistent)] pub v_ed_kn: Option<f64>,
    #[state(persistent)] pub a_mm2: Option<f64>,
    #[state(persistent)] pub a_v_mm2: Option<f64>,
    #[state(persistent)] pub w_pl_mm3: Option<f64>,
    #[state(persistent)] pub f_y_mpa: Option<f64>,
    #[state(persistent)] pub f_u_mpa: Option<f64>,
    #[state(persistent)] pub chi: Option<f64>,
    #[state(persistent)] pub a_net_mm2: Option<f64>,
    #[state(persistent)] pub tension_n_ed_kn: Option<f64>,
    #[state(persistent)] pub fire_thickness_mm: Option<f64>,
    #[state(persistent)] pub fire_rating: Option<String>,
    #[state(persistent)] pub fire_massivity: Option<f64>,
    #[state(persistent)] pub fire_mu_0: Option<f64>,
    #[state(persistent)] pub fire_design_temperature_c: Option<f64>,
    #[state(persistent)] pub cf_b_bar_mm: Option<f64>,
    #[state(persistent)] pub cf_t_mm: Option<f64>,
    #[state(persistent)] pub cf_k_sigma: Option<f64>,
    #[state(persistent)] pub cf_psi: Option<f64>,
    #[state(persistent)] pub cf_n_ed_kn: Option<f64>,
    #[state(persistent)] pub cf_gross_resistance_kn: Option<f64>,
    #[state(persistent)] pub stainless_m_ed_knm: Option<f64>,
    #[state(persistent)] pub stainless_w_pl_mm3: Option<f64>,
    #[state(persistent)] pub stainless_f_y_mpa: Option<f64>,
    #[state(persistent)] pub plated_lambda_p: Option<f64>,
    #[state(persistent)] pub plated_sigma_ed_mpa: Option<f64>,
    #[state(persistent)] pub silo_t_mm: Option<f64>,
    #[state(persistent)] pub silo_r_mm: Option<f64>,
    #[state(persistent)] pub shell_sigma_x_ed_mpa: Option<f64>,
    #[state(persistent)] pub silo_k: Option<f64>,
    #[state(persistent)] pub silo_gamma_kn_m3: Option<f64>,
    #[state(persistent)] pub silo_depth_m: Option<f64>,
    #[state(persistent)] pub bolt_f_ed_kn: Option<f64>,
    #[state(persistent)] pub bolt_n_bolts: Option<u32>,
    #[state(persistent)] pub bolt_a_s_mm2: Option<f64>,
    #[state(persistent)] pub bolt_e1_mm: Option<f64>,
    #[state(persistent)] pub bolt_e2_mm: Option<f64>,
    #[state(persistent)] pub bolt_d0_mm: Option<f64>,
    #[state(persistent)] pub bolt_d_mm: Option<f64>,
    #[state(persistent)] pub bolt_t_mm: Option<f64>,
    #[state(persistent)] pub bolt_f_u_mpa: Option<f64>,
    #[state(persistent)] pub bolt_f_ub_mpa: Option<f64>,
    #[state(persistent)] pub weld_a_mm: Option<f64>,
    #[state(persistent)] pub weld_l_mm: Option<f64>,
    #[state(persistent)] pub weld_f_u_mpa: Option<f64>,
    #[state(persistent)] pub weld_steel_grade: Option<String>,
    #[state(persistent)] pub weld_f_ed_kn: Option<f64>,
    #[state(persistent)] pub delta_sigma_mpa: Option<f64>,
    #[state(persistent)] pub fatigue_category: Option<u8>,
    #[state(persistent)] pub fatigue_method: Option<String>,
    #[state(persistent)] pub t10_steel_subgrade: Option<String>,
    #[state(persistent)] pub t10_actual_thickness_mm: Option<f64>,
    #[state(persistent)] pub t10_t_ed_c: Option<f64>,
    #[state(persistent)] pub tension_component_f_uk_kn: Option<f64>,
    #[state(persistent)] pub tension_component_f_k_kn: Option<f64>,
    #[state(persistent)] pub tension_component_n_ed_kn: Option<f64>,
    #[state(persistent)] pub hss_w_el_mm3: Option<f64>,
    #[state(persistent)] pub hss_f_y_mpa: Option<f64>,
    #[state(persistent)] pub hss_section_class: Option<u8>,
    #[state(persistent)] pub hss_m_ed_knm: Option<f64>,
    #[state(persistent)] pub bridge_lambda: Option<f64>,
    #[state(persistent)] pub bridge_phi_2: Option<f64>,
    #[state(persistent)] pub bridge_delta_sigma_p_mpa: Option<f64>,
    #[state(persistent)] pub tower_wind_factor: Option<f64>,
    #[state(persistent)] pub tower_n_ed_kn: Option<f64>,
    #[state(persistent)] pub pile_sigma_mpa: Option<f64>,
    #[state(persistent)] pub pile_k_red: Option<f64>,
    #[state(persistent)] pub pile_n_ed_kn: Option<f64>,
    #[state(persistent)] pub crane_f_z_ed_kn: Option<f64>,
    #[state(persistent)] pub crane_wheel_contact_length_mm: Option<f64>,
    #[state(persistent)] pub crane_dispersion_mm: Option<f64>,
    #[state(persistent)] pub crane_t_w_mm: Option<f64>,
    #[state(shared_ui)] pub selected_check_index: Option<Option<u32>>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 List wrapper for optional vector diffs.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct En1993StringList { pub values: Vec<String> }
//#endregion 🔖️DeltaHelpers
