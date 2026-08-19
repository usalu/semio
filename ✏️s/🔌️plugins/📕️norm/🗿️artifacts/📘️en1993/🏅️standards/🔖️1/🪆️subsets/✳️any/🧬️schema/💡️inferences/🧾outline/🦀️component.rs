//! 🧾 `outline` — one named inference: this document's own field/section structure. A norm
//! compliance record IS the document it describes, so its "outline" is its top-level field list
//! (`sectionOutline`/`fieldCount`, fixed by the snapshot's own schema shape) plus a real
//! `entryCount` over whatever repeated sub-entries it actually carries (0 when the snapshot has
//! no collection-typed top-level field).

use crate::artifacts::en1993::En1993Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Outline
const SECTION_FIELDS: &[&str] = &[
    "annex",
    "n_ed_kn",
    "m_ed_knm",
    "v_ed_kn",
    "a_mm2",
    "a_v_mm2",
    "w_pl_mm3",
    "f_y_mpa",
    "f_u_mpa",
    "chi",
    "a_net_mm2",
    "tension_n_ed_kn",
    "fire_thickness_mm",
    "fire_rating",
    "fire_massivity",
    "fire_mu_0",
    "fire_design_temperature_c",
    "cf_b_bar_mm",
    "cf_t_mm",
    "cf_k_sigma",
    "cf_psi",
    "cf_n_ed_kn",
    "cf_gross_resistance_kn",
    "stainless_m_ed_knm",
    "stainless_w_pl_mm3",
    "stainless_f_y_mpa",
    "plated_lambda_p",
    "plated_sigma_ed_mpa",
    "silo_t_mm",
    "silo_r_mm",
    "shell_sigma_x_ed_mpa",
    "silo_k",
    "silo_gamma_kn_m3",
    "silo_depth_m",
    "bolt_f_ed_kn",
    "bolt_n_bolts",
    "bolt_a_s_mm2",
    "bolt_e1_mm",
    "bolt_e2_mm",
    "bolt_d0_mm",
    "bolt_d_mm",
    "bolt_t_mm",
    "bolt_f_u_mpa",
    "bolt_f_ub_mpa",
    "weld_a_mm",
    "weld_l_mm",
    "weld_f_u_mpa",
    "weld_steel_grade",
    "weld_f_ed_kn",
    "delta_sigma_mpa",
    "fatigue_category",
    "fatigue_method",
    "t10_steel_subgrade",
    "t10_actual_thickness_mm",
    "t10_t_ed_c",
    "tension_component_f_uk_kn",
    "tension_component_f_k_kn",
    "tension_component_n_ed_kn",
    "hss_w_el_mm3",
    "hss_f_y_mpa",
    "hss_section_class",
    "hss_m_ed_knm",
    "bridge_lambda",
    "bridge_phi_2",
    "bridge_delta_sigma_p_mpa",
    "tower_wind_factor",
    "tower_n_ed_kn",
    "pile_sigma_mpa",
    "pile_k_red",
    "pile_n_ed_kn",
    "crane_f_z_ed_kn",
    "crane_wheel_contact_length_mm",
    "crane_dispersion_mm",
    "crane_t_w_mm",
];

/// 🧾️ `En1993` document outline.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct En1993Outline {
    pub section_outline: Vec<String>,
    pub field_count: u32,
    pub entry_count: u32,
}

impl En1993Outline {
    pub async fn compute(_snapshot: &En1993Snapshot) -> Self {
        let section_outline: Vec<String> = SECTION_FIELDS.iter().map(|s| s.to_string()).collect();
        let field_count = section_outline.len() as u32;
        let entry_count = 0;
        Self { section_outline, field_count, entry_count }
    }
}

impl Default for En1993Outline {
    fn default() -> Self {
        Self::compute(&En1993Snapshot::default())
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn outline_field_count_matches_section_outline_length() {
        let outline = En1993Outline::compute(&En1993Snapshot::default());
        assert_eq!(outline.field_count as usize, outline.section_outline.len());
    }

    #[semio_framework_async_macros::async_test]
    async fn outline_is_deterministic() {
        let snapshot = En1993Snapshot::default();
        assert_eq!(En1993Outline::compute(&snapshot), En1993Outline::compute(&snapshot));
    }
}
//#endregion 🧪️Tests
