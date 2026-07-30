//! 🧱 EN 1992 design of concrete structures — document entities (constitutional: general).

use norm_core::AnnexChoice;
use serde::{Deserialize, Serialize};

//#region 🔖Types
pub mod part_1_2 {
    use super::*;

    /// 🏗️ Fire resistance rating.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
    pub enum FireRating {
        R30,
        R60,
        R90,
        R120,
    }
}

pub mod part_3 {
    use super::*;

    /// 💧 Tightness class per EN 1992-3 Table 7.105: required degree of protection against leakage.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
    pub enum TightnessClass {
        Tc0,
        Tc1,
        Tc2,
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "en1992", layout = "lines")]
pub struct Document {
    pub annex: AnnexChoice,
    pub m_ed_knm: f64,
    pub v_ed_kn: f64,
    pub f_ck: f64,
    pub b_mm: f64,
    pub d_mm: f64,
    pub a_s_mm2: f64,
    pub f_yk: f64,
    pub rho_l: f64,
    pub n_ed_kn: f64,
    pub p_kn: f64,
    pub a_c_mm2: f64,
    pub use_fem: bool,
    pub span_m: f64,
    pub udl_kn_m: f64,
    pub fire_rating: part_1_2::FireRating,
    pub provided_axis_distance_mm: f64,
    pub bridge_sigma_c_mpa: f64,
    pub bridge_delta_sigma_s_mpa: f64,
    pub tightness_class: part_3::TightnessClass,
    pub hd_over_h: f64,
    pub liquid_sigma_s_mpa: f64,
    pub liquid_rho_p_eff: f64,
    pub liquid_f_ct_eff_mpa: f64,
    pub liquid_e_s_mpa: f64,
    pub liquid_s_r_max_mm: f64,
    pub anchor_h_ef_mm: f64,
    pub anchor_cracked: bool,
    pub anchor_f_uk_mpa: f64,
    pub anchor_f_yk_mpa: f64,
    pub anchor_a_s_mm2: f64,
    pub anchor_d_mm: f64,
    pub anchor_c1_mm: f64,
    pub anchor_n_ed_kn: f64,
    pub anchor_v_ed_kn: f64,
}

impl Default for Document {
    fn default() -> Self {
        Self {
            annex: AnnexChoice::De,
            m_ed_knm: 120.0,
            v_ed_kn: 80.0,
            f_ck: 30.0,
            b_mm: 300.0,
            d_mm: 450.0,
            a_s_mm2: 1200.0,
            f_yk: 500.0,
            rho_l: 0.01,
            n_ed_kn: 0.0,
            p_kn: 0.0,
            a_c_mm2: 135_000.0,
            use_fem: false,
            span_m: 6.0,
            udl_kn_m: 20.0,
            fire_rating: part_1_2::FireRating::R60,
            provided_axis_distance_mm: 30.0,
            bridge_sigma_c_mpa: 12.0,
            bridge_delta_sigma_s_mpa: 100.0,
            tightness_class: part_3::TightnessClass::Tc1,
            hd_over_h: 10.0,
            liquid_sigma_s_mpa: 200.0,
            liquid_rho_p_eff: 0.01,
            liquid_f_ct_eff_mpa: 2.9,
            liquid_e_s_mpa: 200_000.0,
            liquid_s_r_max_mm: 250.0,
            anchor_h_ef_mm: 80.0,
            anchor_cracked: false,
            anchor_f_uk_mpa: 800.0,
            anchor_f_yk_mpa: 640.0,
            anchor_a_s_mm2: 84.3,
            anchor_d_mm: 12.0,
            anchor_c1_mm: 100.0,
            anchor_n_ed_kn: 10.0,
            anchor_v_ed_kn: 5.0,
        }
    }
}
//#endregion 🔖Types
