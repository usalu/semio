//! 🌬️ EN 1991 actions on structures — document entities (constitutional: general).

use crate::core::{AnnexChoice, ImposedCategory};
use serde::{Deserialize, Serialize};

//#region 🔖️Types
pub mod part_1_2 {
    use super::*;

    /// 🔥️ Nominal fire exposure curve per EN 1991-1-2 §3.2/Annex B.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
    pub enum FireCurve {
        Standard,
        External,
        Hydrocarbon,
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "norm.en1991", layout = "lines")]
pub struct Document {
    #[dsl(unit = "m2")]
    pub area_m2: f64,
    pub category: ImposedCategory,
    pub annex: AnnexChoice,
    pub self_weight_material: String,
    #[dsl(unit = "m")]
    pub self_weight_thickness_m: f64,
    #[dsl(unit = "kN/m2")]
    pub assumed_g_k_kn_m2: f64,
    pub fire_curve: part_1_2::FireCurve,
    pub fire_resistance_min: f64,
    pub fire_member_capacity_c: f64,
    pub snow_zone: u8,
    #[dsl(unit = "m")]
    pub snow_altitude_m: f64,
    #[dsl(unit = "kN/m2")]
    pub en_s_k_kn_m2: f64,
    pub wind_zone: u8,
    #[dsl(unit = "m/s")]
    pub en_v_b_m_s: f64,
    #[dsl(unit = "K")]
    pub delta_t_k: f64,
    pub construction_activity: String,
    #[dsl(unit = "t")]
    pub accidental_mass_t: f64,
    pub accidental_speed_km_h: f64,
    pub bridge_lane: u8,
    #[dsl(unit = "m")]
    pub bridge_span_m: f64,
    #[dsl(unit = "m")]
    pub bridge_lane_width_m: f64,
    pub bridge_moment_resistance_knm: f64,
    pub crane_class: String,
    pub hoist_class: String,
    #[dsl(unit = "m/s")]
    pub hoisting_speed_m_s: f64,
    pub silo_bulk_density_kn_m3: f64,
    #[dsl(unit = "m")]
    pub silo_height_m: f64,
    #[dsl(unit = "m")]
    pub silo_hydraulic_radius_m: f64,
    pub silo_mu: f64,
    pub silo_k: f64,
    pub c_s: f64,
    pub c_d: f64,
}

impl Default for Document {
    fn default() -> Self {
        Self {
            area_m2: 50.0,
            category: ImposedCategory::B,
            annex: AnnexChoice::De,
            self_weight_material: "reinforced_concrete".into(),
            self_weight_thickness_m: 0.2,
            assumed_g_k_kn_m2: 6.0,
            fire_curve: part_1_2::FireCurve::Standard,
            fire_resistance_min: 30.0,
            fire_member_capacity_c: 900.0,
            snow_zone: 2,
            snow_altitude_m: 150.0,
            en_s_k_kn_m2: 0.85,
            wind_zone: 2,
            en_v_b_m_s: 25.0,
            delta_t_k: 30.0,
            construction_activity: "scaffolding".into(),
            accidental_mass_t: 30.0,
            accidental_speed_km_h: 80.0,
            bridge_lane: 1,
            bridge_span_m: 20.0,
            bridge_lane_width_m: 3.0,
            bridge_moment_resistance_knm: 3000.0,
            crane_class: "HC2".into(),
            hoist_class: "HC2".into(),
            hoisting_speed_m_s: 0.5,
            silo_bulk_density_kn_m3: 8.0,
            silo_height_m: 12.0,
            silo_hydraulic_radius_m: 1.5,
            silo_mu: 0.4,
            silo_k: 0.4,
            c_s: 1.0,
            c_d: 1.0,
        }
    }
}
//#endregion 🔖️Types

//#region 🔖️ArtifactKind
/// 🗿️ The computed-compliance artifact this standard publishes on its app's `report:out` port —
/// lifted out of the pre-migration manifest's inline `.artifact_kind(ArtifactKindSpec { .. })` so the
/// artifact node, not the app, owns its own kind declaration.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    crate::core::app::artifact_kind_spec("en1991", "EN 1991")
}
//#endregion 🔖️ArtifactKind
