//! ⚙️ EN 1991 actions on structures — headless compute (constitutional: engine).

use crate::artifacts::en1990::engine::{na_de::NaDe, na_en::NaEn};
use crate::artifacts::en1991::mutations::En1991Mutation;
use crate::artifacts::en1991::{part_1_2::FireCurve, Document};
use crate::document::{AnnexChoice, CheckReport, CheckResult, ClauseId, ImposedCategory, NationalAnnex, NormFamily, NormFamilyId, NormHost, Quantity};

// #region 🔖️NaDe
pub mod na_de {
    pub use crate::artifacts::en1990::engine::na_de::NaDe;

    /// ❄️ German snow zone per DIN EN 1991-1-3/NA.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SnowZone {
        Zone1,
        Zone2,
        Zone3,
    }

    impl SnowZone {
        pub fn as_u8(self) -> u8 {
            match self {
                Self::Zone1 => 1,
                Self::Zone2 => 2,
                Self::Zone3 => 3,
            }
        }

        pub fn s_k_kn_m2(self) -> f64 {
            match self {
                Self::Zone1 => 0.65,
                Self::Zone2 => 0.85,
                Self::Zone3 => 1.10,
            }
        }
    }

    /// 🌬️ German wind zone per DIN EN 1991-1-4/NA.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum WindZone {
        Zone1,
        Zone2,
        Zone3,
        Zone4,
    }

    impl WindZone {
        pub fn v_b_m_s(self) -> f64 {
            match self {
                Self::Zone1 => 22.5,
                Self::Zone2 => 25.0,
                Self::Zone3 => 27.5,
                Self::Zone4 => 30.0,
            }
        }
    }

    pub fn ground_snow_load(zone: SnowZone) -> f64 {
        zone.s_k_kn_m2()
    }

    pub fn basic_wind_velocity(zone: WindZone) -> f64 {
        zone.v_b_m_s()
    }
}
// #endregion 🔖️NaDe

// #region 🔖️Part1_1
pub mod part_1_1 {
    use super::*;

    /// 🧱️ Unit weight [kN/m³] per EN 1991-1-1 Annex A.
    pub fn self_weight_kn_m3(material: &str) -> f64 {
        match material {
            "concrete" => 25.0,
            "reinforced_concrete" => 25.0,
            "steel" => 78.5,
            "timber" => 5.0,
            "glulam" => 4.2,
            "masonry" => 18.0,
            "brick" => 20.0,
            "aluminium" => 27.0,
            "glass" => 25.0,
            "water" => 10.0,
            "sand" => 18.0,
            "gravel" => 20.0,
            "asphalt" => 23.0,
            _ => 20.0,
        }
    }

    /// 🧱️ Self-weight per unit area [kN/m²] of a layer of given thickness.
    pub fn self_weight_kn_m2(material: &str, thickness_m: f64) -> f64 {
        self_weight_kn_m3(material) * thickness_m
    }

    pub fn imposed_load_kn_m2(category: ImposedCategory) -> f64 {
        category.q_k_kn_m2()
    }

    pub fn check_imposed(area_m2: f64, category: ImposedCategory, annex: &dyn NationalAnnex) -> CheckResult {
        let q = imposed_load_kn_m2(category) * area_m2;
        let psi = annex.psi_0(category.label());
        CheckResult::from_utilization(ClauseId::new("EN 1991-1-1", "Table 6.1", "q"), Quantity::force_kn(q * psi), Quantity::force_kn(q), "imposed load", annex.choice())
    }

    /// ✅️ Verify the assumed design dead load covers the material self-weight.
    pub fn check_self_weight(material: &str, thickness_m: f64, assumed_g_k_kn_m2: f64, annex: AnnexChoice) -> CheckResult {
        let g_k = self_weight_kn_m2(material, thickness_m);
        CheckResult::from_utilization(
            ClauseId::new("EN 1991-1-1", "Annex A", "A.1"),
            Quantity::new(crate::document::QuantityKind::Pressure, g_k * 1000.0),
            Quantity::new(crate::document::QuantityKind::Pressure, assumed_g_k_kn_m2 * 1000.0),
            "self-weight vs assumed dead load",
            annex,
        )
    }
}
// #endregion 🔖️Part1_1

// #region 🔖️Part1_2
pub mod part_1_2 {
    use super::*;

    /// 🔥️ ISO 834 standard temperature-time curve θ_g [°C], EN 1991-1-2 Eq. 3.4.
    pub fn standard_gas_temperature_c(t_min: f64) -> f64 {
        20.0 + 345.0 * (8.0 * t_min.max(0.0) + 1.0).log10()
    }

    /// 🔥️ External fire curve θ_g [°C], EN 1991-1-2 Annex B Eq. B.4.
    pub fn external_gas_temperature_c(t_min: f64) -> f64 {
        660.0 * (1.0 - 0.687 * (-0.32 * t_min).exp() - 0.313 * (-3.8 * t_min).exp()) + 20.0
    }

    /// 🔥️ Hydrocarbon fire curve θ_g [°C], EN 1991-1-2 Annex B Eq. B.5.
    pub fn hydrocarbon_gas_temperature_c(t_min: f64) -> f64 {
        1080.0 * (1.0 - 0.325 * (-0.167 * t_min).exp() - 0.675 * (-2.5 * t_min).exp()) + 20.0
    }

    pub fn gas_temperature_c(curve: FireCurve, t_min: f64) -> f64 {
        match curve {
            FireCurve::Standard => standard_gas_temperature_c(t_min),
            FireCurve::External => external_gas_temperature_c(t_min),
            FireCurve::Hydrocarbon => hydrocarbon_gas_temperature_c(t_min),
        }
    }

    /// ✅️ Verify the member's rated fire-resistance temperature capacity exceeds the gas temperature at t_min.
    pub fn check_fire_action(curve: FireCurve, t_min: f64, member_capacity_c: f64, annex: AnnexChoice) -> CheckResult {
        let theta_g = gas_temperature_c(curve, t_min);
        CheckResult::from_utilization(ClauseId::new("EN 1991-1-2", "§3.2", "3.4"), Quantity::new(crate::document::QuantityKind::Temperature, theta_g), Quantity::new(crate::document::QuantityKind::Temperature, member_capacity_c), "fire gas temperature", annex)
    }
}
// #endregion 🔖️Part1_2

// #region 🔖️Part1_3
pub mod part_1_3 {
    use super::*;

    pub fn ground_snow_load_zone(zone: u8) -> f64 {
        match zone {
            1 => na_de::SnowZone::Zone1.s_k_kn_m2(),
            2 => na_de::SnowZone::Zone2.s_k_kn_m2(),
            3 => na_de::SnowZone::Zone3.s_k_kn_m2(),
            _ => na_de::SnowZone::Zone2.s_k_kn_m2(),
        }
    }

    pub fn roof_snow_load(s_k: f64, mu: f64) -> f64 {
        mu * s_k
    }

    pub fn altitude_correction(s_k: f64, altitude_m: f64, zone: u8) -> f64 {
        let delta_h = match zone {
            1 => 150.0,
            2 => 200.0,
            3 => 250.0,
            _ => 200.0,
        };
        if altitude_m <= delta_h {
            s_k
        } else {
            s_k * (1.0 + 0.001 * (altitude_m - delta_h).max(0.0))
        }
    }

    /// ❄️ Characteristic ground snow load: DE zone/altitude formula vs EN user-supplied s_k (NDP EN 1991-1-3/NA §4.1).
    pub fn design_ground_snow_load(annex: AnnexChoice, zone: u8, altitude_m: f64, en_s_k_kn_m2: f64) -> f64 {
        match annex {
            AnnexChoice::De => altitude_correction(ground_snow_load_zone(zone), altitude_m, zone),
            AnnexChoice::En => en_s_k_kn_m2,
        }
    }

    pub fn check_snow(s_kn_m2: f64, limit: f64, annex: &dyn NationalAnnex) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1991-1-3", "§5", "5.1"), Quantity::new(crate::document::QuantityKind::Pressure, s_kn_m2 * 1000.0), Quantity::new(crate::document::QuantityKind::Pressure, limit * 1000.0), "snow load", annex.choice())
    }
}
// #endregion 🔖️Part1_3

// #region 🔖️Part1_4
pub mod part_1_4 {
    use super::*;

    /// 🌬️ Terrain category per EN 1991-1-4 Table 4.1.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum TerrainCategory {
        Zero,
        I,
        II,
        III,
        IV,
    }

    impl TerrainCategory {
        pub fn z_0_m(self) -> f64 {
            match self {
                Self::Zero => 0.003,
                Self::I => 0.01,
                Self::II => 0.05,
                Self::III => 0.3,
                Self::IV => 1.0,
            }
        }

        pub fn z_min_m(self) -> f64 {
            match self {
                Self::Zero => 1.0,
                Self::I => 1.0,
                Self::II => 2.0,
                Self::III => 5.0,
                Self::IV => 10.0,
            }
        }
    }

    /// 🌬️ Basic velocity pressure q_b [kN/m²] from v_b.
    pub fn basic_velocity_pressure(rho: f64, v_b_m_s: f64) -> f64 {
        0.5 * rho * v_b_m_s * v_b_m_s / 1000.0
    }

    /// 🌬️ Peak velocity pressure q_p [kN/m²] per EN 1991-1-4 Eq. (4.8).
    pub fn peak_velocity_pressure(rho: f64, v_b_m_s: f64, c_e: f64) -> f64 {
        c_e * basic_velocity_pressure(rho, v_b_m_s)
    }

    pub fn exposure_factor(z_m: f64, terrain: TerrainCategory) -> f64 {
        let z = z_m.max(terrain.z_min_m());
        let z_0 = terrain.z_0_m();
        let k_r = 0.19 * (z_0 / 0.05_f64).powf(0.07);
        let c_0 = k_r * (z / z_0).ln();
        c_0 * c_0
    }

    pub fn wind_pressure(q_p: f64, c_pe: f64, c_pi: f64) -> f64 {
        q_p * (c_pe - c_pi)
    }

    pub fn structural_factor(c_s: f64, c_d: f64) -> f64 {
        c_s * c_d
    }

    /// 🌬️ Basic wind velocity v_b: DE wind-zone table vs EN user-supplied value (NDP EN 1991-1-4/NA §4.2).
    pub fn design_basic_wind_velocity(annex: AnnexChoice, zone: u8, en_v_b_m_s: f64) -> f64 {
        match annex {
            AnnexChoice::De => match zone {
                1 => na_de::WindZone::Zone1.v_b_m_s(),
                2 => na_de::WindZone::Zone2.v_b_m_s(),
                3 => na_de::WindZone::Zone3.v_b_m_s(),
                _ => na_de::WindZone::Zone4.v_b_m_s(),
            },
            AnnexChoice::En => en_v_b_m_s,
        }
    }

    pub fn check_wind(w_p_kn_m2: f64, limit: f64, annex: &dyn NationalAnnex) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1991-1-4", "§5", "5.1"), Quantity::new(crate::document::QuantityKind::Pressure, w_p_kn_m2 * 1000.0), Quantity::new(crate::document::QuantityKind::Pressure, limit * 1000.0), "wind pressure", annex.choice())
    }
}
// #endregion 🔖️Part1_4

// #region 🔖️Part1_5
pub mod part_1_5 {
    use super::*;

    pub fn thermal_coefficient_alpha_k_inv() -> f64 {
        1.0e-5
    }

    pub fn temperature_difference_action(delta_t_k: f64, alpha: f64, e_modulus_gpa: f64) -> f64 {
        alpha * delta_t_k * e_modulus_gpa
    }

    pub fn check_temperature_action(delta_t_k: f64, limit_k: f64) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1991-1-5", "§6", "6.1"), Quantity::new(crate::document::QuantityKind::Temperature, delta_t_k), Quantity::new(crate::document::QuantityKind::Temperature, limit_k), "thermal action", AnnexChoice::De)
    }

    pub fn check_fire_boundary_temperature(t_surface_k: f64, t_limit_k: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1991-1-5", "Annex A", "A.1"),
            Quantity::new(crate::document::QuantityKind::Temperature, t_surface_k),
            Quantity::new(crate::document::QuantityKind::Temperature, t_limit_k),
            "fire boundary temperature",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖️Part1_5

// #region 🔖️Part1_6
pub mod part_1_6 {
    use super::*;

    pub fn construction_load_kn_m2(activity: &str) -> f64 {
        match activity {
            "storage" => 2.0,
            "machinery" => 3.0,
            "scaffolding" => 1.0,
            _ => 0.5,
        }
    }

    pub fn check_construction_load(q_kn_m2: f64, limit: f64) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1991-1-6", "§4", "4.1"), Quantity::force_kn(q_kn_m2), Quantity::force_kn(limit), "construction load", AnnexChoice::En)
    }
}
// #endregion 🔖️Part1_6

// #region 🔖️Part1_7
pub mod part_1_7 {
    use super::*;

    pub fn impact_force_kn(vehicle_mass_t: f64, speed_km_h: f64) -> f64 {
        0.5 * vehicle_mass_t * (speed_km_h / 3.6).powi(2) / 1000.0
    }

    pub fn explosion_pressure_kpa(mass_kg: f64, distance_m: f64) -> f64 {
        if distance_m < f64::EPSILON {
            return 0.0;
        }
        2.0 * mass_kg / (distance_m * distance_m)
    }

    pub fn check_accidental_pressure(p_kpa: f64, limit_kpa: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1991-1-7", "Annex B", "B.1"),
            Quantity::new(crate::document::QuantityKind::Pressure, p_kpa * 1000.0),
            Quantity::new(crate::document::QuantityKind::Pressure, limit_kpa * 1000.0),
            "accidental pressure",
            AnnexChoice::En,
        )
    }
}
// #endregion 🔖️Part1_7

// #region 🔖️Part2
pub mod part_2 {
    use super::*;

    pub fn lm1_udl_kn_m2(lane: u8) -> f64 {
        match lane {
            1 => 9.0,
            2 => 2.5,
            _ => 2.5,
        }
    }

    pub fn lm1_tandem_kn(lane: u8) -> f64 {
        match lane {
            1 => 300.0,
            2 => 200.0,
            _ => 200.0,
        }
    }

    /// 🌉️ α adjustment factor for LM1 tandem/UDL: DE-NA reduces lane 1 vs EN recommended 1.0 (DIN EN 1991-2/NA §4.3.2).
    pub fn alpha_q(annex: AnnexChoice, lane: u8) -> f64 {
        match (annex, lane) {
            (AnnexChoice::De, 1) => 0.9,
            (AnnexChoice::De, _) => 1.0,
            (AnnexChoice::En, _) => 1.0,
        }
    }

    /// 🌉️ Design tandem-system axle load [kN] including α_Q adjustment.
    pub fn lm1_design_tandem_kn(annex: AnnexChoice, lane: u8) -> f64 {
        alpha_q(annex, lane) * lm1_tandem_kn(lane)
    }

    /// 🌉️ Simply-supported mid-span bending moment [kNm] from LM1 tandem + UDL over a span.
    pub fn mid_span_moment_knm(span_m: f64, tandem_kn: f64, udl_kn_m2: f64, lane_width_m: f64) -> f64 {
        tandem_kn * span_m / 4.0 + udl_kn_m2 * lane_width_m * span_m * span_m / 8.0
    }

    pub fn check_imposed_bridge(lane_load_kn: f64, design_kn: f64) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1991-2", "§4", "4.3"), Quantity::force_kn(design_kn), Quantity::force_kn(lane_load_kn), "bridge imposed load", AnnexChoice::En)
    }

    /// ✅️ Check LM1-derived mid-span moment against section resistance.
    pub fn check_lm1_moment(annex: AnnexChoice, span_m: f64, lane: u8, lane_width_m: f64, resistance_knm: f64) -> CheckResult {
        let tandem = lm1_design_tandem_kn(annex, lane);
        let m_ed = mid_span_moment_knm(span_m, tandem, lm1_udl_kn_m2(lane), lane_width_m);
        CheckResult::from_utilization(ClauseId::new("EN 1991-2", "§4.3.2", "4.4"), Quantity::new(crate::document::QuantityKind::Moment, m_ed * 1000.0), Quantity::new(crate::document::QuantityKind::Moment, resistance_knm * 1000.0), "LM1 mid-span moment", annex)
    }
}
// #endregion 🔖️Part2

// #region 🔖️Part3
pub mod part_3 {
    use super::*;

    pub fn crane_vertical_wheel_load(crane_class: &str) -> f64 {
        match crane_class {
            "HC1" => 50.0,
            "HC2" => 100.0,
            "HC3" => 160.0,
            "HC4" => 250.0,
            _ => 80.0,
        }
    }

    pub fn crane_horizontal_force_kn(vertical_load_kn: f64) -> f64 {
        0.1 * vertical_load_kn
    }

    /// 🏗️ Hoisting dynamic factor φ_2 per EN 1991-3 Table 2.4 (φ_2,min + β_2·v_h).
    pub fn phi_2(hoist_class: &str, hoisting_speed_m_s: f64) -> f64 {
        let (phi_2_min, beta_2) = match hoist_class {
            "HC1" => (1.05, 0.17),
            "HC2" => (1.10, 0.34),
            "HC3" => (1.15, 0.51),
            _ => (1.20, 0.68),
        };
        phi_2_min + beta_2 * hoisting_speed_m_s
    }

    /// 🏗️ Hoisting dynamic factor φ_1 per EN 1991-3 §2.4.2.1 (self-weight lift-off).
    pub const PHI_1: f64 = 1.1;

    /// 🏗️ Design vertical wheel load [kN] including hoisting dynamics.
    pub fn design_vertical_wheel_load(crane_class: &str, hoist_class: &str, hoisting_speed_m_s: f64) -> f64 {
        crane_vertical_wheel_load(crane_class) * PHI_1.max(phi_2(hoist_class, hoisting_speed_m_s))
    }

    pub fn check_crane_load(wheel_load_kn: f64, capacity_kn: f64) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1991-3", "§2", "2.3"), Quantity::force_kn(wheel_load_kn), Quantity::force_kn(capacity_kn), "crane wheel load", AnnexChoice::En)
    }
}
// #endregion 🔖️Part3

// #region 🔖️Part4
pub mod part_4 {
    use super::*;

    /// 🌾️ Janssen horizontal wall pressure p_h(z) [kPa] per EN 1991-4 Annex C Eq. C.4 (asymptotic silo pressure).
    pub fn janssen_horizontal_pressure_kpa(bulk_density_kn_m3: f64, hydraulic_radius_m: f64, mu: f64, k: f64, depth_m: f64) -> f64 {
        let asymptote = bulk_density_kn_m3 * hydraulic_radius_m / (mu * k);
        asymptote * (1.0 - (-depth_m * mu * k / hydraulic_radius_m).exp())
    }

    /// 🌾️ Legacy linear wall pressure surrogate, retained for simple hand checks.
    pub fn silo_wall_pressure_kpa(bulk_density_kn_m3: f64, height_m: f64, k: f64) -> f64 {
        k * bulk_density_kn_m3 * height_m
    }

    pub fn tank_hydrostatic_pressure_kpa(fluid_density_kn_m3: f64, fill_height_m: f64) -> f64 {
        fluid_density_kn_m3 * fill_height_m
    }

    pub fn check_silo_pressure(p_kpa: f64, limit_kpa: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1991-4", "§5", "5.1"),
            Quantity::new(crate::document::QuantityKind::Pressure, p_kpa * 1000.0),
            Quantity::new(crate::document::QuantityKind::Pressure, limit_kpa * 1000.0),
            "silo wall pressure",
            AnnexChoice::En,
        )
    }
}
// #endregion 🔖️Part4

/// 📋️ Aggregate action checks for a typical floor bay.
pub fn check_floor_actions(area_m2: f64, category: ImposedCategory, wind_zone_vb: f64, snow_zone: u8, use_de_na: bool) -> CheckReport {
    let annex: &dyn NationalAnnex = if use_de_na { &NaDe } else { &NaEn };
    let mut report = CheckReport::default();
    report.push(part_1_1::check_imposed(area_m2, category, annex));
    let c_e = part_1_4::exposure_factor(10.0, part_1_4::TerrainCategory::II);
    let q_p = part_1_4::peak_velocity_pressure(1.25, wind_zone_vb, c_e);
    report.push(part_1_4::check_wind(part_1_4::wind_pressure(q_p, 0.8, 0.2), 1.5, annex));
    let s = part_1_3::roof_snow_load(part_1_3::ground_snow_load_zone(snow_zone), 0.8);
    report.push(part_1_3::check_snow(s, 1.2, annex));
    report
}

/// 📋️ Full EN 1991 action checks across parts 1-1 through 1-7 and parts 2–4.
pub fn check_full_actions(document: &Document) -> CheckReport {
    let annex: &dyn NationalAnnex = if document.annex == AnnexChoice::De { &NaDe } else { &NaEn };
    let mut report = CheckReport::default();
    report.push(part_1_1::check_imposed(document.area_m2, document.category, annex));
    report.push(part_1_1::check_self_weight(&document.self_weight_material, document.self_weight_thickness_m, document.assumed_g_k_kn_m2, document.annex));
    report.push(part_1_2::check_fire_action(document.fire_curve, document.fire_resistance_min, document.fire_member_capacity_c, document.annex));
    let s_k = part_1_3::design_ground_snow_load(document.annex, document.snow_zone, document.snow_altitude_m, document.en_s_k_kn_m2);
    let s = part_1_3::roof_snow_load(s_k, 0.8);
    report.push(part_1_3::check_snow(s, 1.2, annex));
    let v_b = part_1_4::design_basic_wind_velocity(document.annex, document.wind_zone, document.en_v_b_m_s);
    let c_e = part_1_4::exposure_factor(10.0, part_1_4::TerrainCategory::II);
    let q_p = part_1_4::peak_velocity_pressure(1.25, v_b, c_e);
    let c_sc_d = part_1_4::structural_factor(document.c_s, document.c_d);
    let w_p = part_1_4::wind_pressure(q_p, 0.8, 0.2) * c_sc_d;
    report.push(part_1_4::check_wind(w_p, 1.5, annex));
    report.push(part_1_5::check_temperature_action(document.delta_t_k, 50.0));
    let q_const = part_1_6::construction_load_kn_m2(&document.construction_activity);
    report.push(part_1_6::check_construction_load(q_const, 5.0));
    let impact = part_1_7::impact_force_kn(document.accidental_mass_t, document.accidental_speed_km_h);
    report.push(CheckResult::from_utilization(ClauseId::new("EN 1991-1-7", "Annex B", "B.2"), Quantity::force_kn(impact), Quantity::force_kn(500.0), "accidental impact", annex.choice()));
    report.push(part_2::check_lm1_moment(document.annex, document.bridge_span_m, document.bridge_lane, document.bridge_lane_width_m, document.bridge_moment_resistance_knm));
    let wheel = part_3::design_vertical_wheel_load(&document.crane_class, &document.hoist_class, document.hoisting_speed_m_s);
    report.push(part_3::check_crane_load(wheel, wheel * 1.2));
    let silo_p = part_4::janssen_horizontal_pressure_kpa(document.silo_bulk_density_kn_m3, document.silo_hydraulic_radius_m, document.silo_mu, document.silo_k, document.silo_height_m);
    report.push(part_4::check_silo_pressure(silo_p, 100.0));
    report
}

// #region 🔖️Session

//#region 🔖️ArtifactEngine
/// @emoji ⚙️ UI-independent En1991 artifact engine — owns the projection; every transition is a mutation.
pub struct En1991Engine {
    projection: Document,
}

impl En1991Engine {
    pub fn new(projection: Document) -> Self {
        Self { projection }
    }

    pub fn into_projection(self) -> Document {
        self.projection
    }
}

impl protocol::ArtifactEngine for En1991Engine {
    type Projection = Document;
    type Mutation = En1991Mutation;
    type Diff = crate::artifacts::en1991::diff::Diff;

    fn projection(&self) -> &Self::Projection {
        &self.projection
    }

    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {
        let diff = protocol::Mutation::diff(mutation, &self.projection);
        self.projection = vcs::apply_mutation(&self.projection, mutation);
        Ok(diff)
    }

    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {
        protocol::Mutation::inverse(mutation, &self.projection)
    }
}
//#endregion 🔖️ArtifactEngine

pub type Host = NormHost<En1991Family>;

pub fn evaluate(document: &Document) -> CheckReport {
    check_full_actions(document)
}

pub struct En1991Family;

impl NormFamily for En1991Family {
    type Document = Document;
    type Mutation = En1991Mutation;

    fn family_id() -> NormFamilyId {
        NormFamilyId::En1991
    }

    fn evaluate(document: &Document) -> CheckReport {
        evaluate(document)
    }
}
// #endregion 🔖️Session

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snow_zone_2_ground_load() {
        assert!((part_1_3::ground_snow_load_zone(2) - 0.85).abs() < 1e-9);
        assert!((na_de::SnowZone::Zone2.s_k_kn_m2() - 0.85).abs() < 1e-9);
    }

    #[test]
    fn wind_peak_velocity_pressure_vb_25() {
        let q_b = part_1_4::basic_velocity_pressure(1.25, 25.0);
        assert!((q_b - 0.39).abs() < 0.01);
        let c_e = part_1_4::exposure_factor(10.0, part_1_4::TerrainCategory::II);
        let q_p = part_1_4::peak_velocity_pressure(1.25, 25.0, c_e);
        assert!(q_p > q_b);
    }

    #[test]
    fn imposed_categories_table_6_1() {
        assert_eq!(part_1_1::imposed_load_kn_m2(ImposedCategory::A), 2.0);
        assert_eq!(part_1_1::imposed_load_kn_m2(ImposedCategory::B), 2.5);
        assert_eq!(part_1_1::imposed_load_kn_m2(ImposedCategory::H), 20.0);
    }

    #[test]
    fn full_actions_de_na_numeric() {
        let doc = Document::default();
        let annex = NaDe;
        let report = check_full_actions(&doc);
        assert_eq!(report.checks.len(), 11);
        let imposed_q = part_1_1::imposed_load_kn_m2(ImposedCategory::B) * doc.area_m2 * annex.psi_0("office");
        assert!((report.checks[0].computed.value / 1000.0 - imposed_q).abs() < 1e-6);
        assert!((report.checks[1].computed.value / 1000.0 - 5.0).abs() < 1e-6);
        let theta_g = part_1_2::standard_gas_temperature_c(30.0);
        assert!((theta_g - 841.79588).abs() < 1e-4);
        assert!((report.checks[2].computed.value - theta_g).abs() < 1e-6);
        let snow = part_1_3::roof_snow_load(part_1_3::ground_snow_load_zone(2), 0.8);
        assert!((report.checks[3].computed.value - snow * 1000.0).abs() < 1e-6);
        let c_e = part_1_4::exposure_factor(10.0, part_1_4::TerrainCategory::II);
        let q_p = part_1_4::peak_velocity_pressure(1.25, 25.0, c_e);
        let w_p = part_1_4::wind_pressure(q_p, 0.8, 0.2) * part_1_4::structural_factor(1.0, 1.0);
        assert!((report.checks[4].computed.value - w_p * 1000.0).abs() < 1e-3);
        assert!((report.checks[5].computed.value - doc.delta_t_k).abs() < 1e-6);
        assert!((report.checks[6].computed.value / 1000.0 - 1.0).abs() < 1e-6);
        let impact = part_1_7::impact_force_kn(30.0, 80.0);
        assert!((report.checks[7].computed.value / 1000.0 - impact).abs() < 1e-6);
        assert!((impact - 7.407407407407407).abs() < 1e-6);
        assert!((report.checks[8].computed.value / 1000.0 - 2700.0).abs() < 1e-6);
        let silo_p = part_4::janssen_horizontal_pressure_kpa(8.0, 1.5, 0.4, 0.4, 12.0);
        assert!((silo_p - 54.147).abs() < 1e-2);
        assert!((report.checks[10].computed.value - silo_p * 1000.0).abs() < 1e-6);
        assert!(report.all_pass());
    }

    #[test]
    fn de_wind_zone_2_basic_velocity() {
        assert!((na_de::WindZone::Zone2.v_b_m_s() - 25.0).abs() < 1e-9);
    }

    #[test]
    fn snow_and_wind_de_vs_en_diverge_at_altitude() {
        let doc = Document { snow_altitude_m: 400.0, annex: AnnexChoice::De, ..Document::default() };
        let de_s_k = part_1_3::design_ground_snow_load(doc.annex, doc.snow_zone, doc.snow_altitude_m, doc.en_s_k_kn_m2);
        let en_s_k = part_1_3::design_ground_snow_load(AnnexChoice::En, doc.snow_zone, doc.snow_altitude_m, doc.en_s_k_kn_m2);
        assert!(de_s_k > en_s_k);
        assert!((en_s_k - doc.en_s_k_kn_m2).abs() < 1e-9);
    }

    #[test]
    fn bridge_lm1_alpha_q_diverges_de_vs_en() {
        let de = part_2::check_lm1_moment(AnnexChoice::De, 20.0, 1, 3.0, 3000.0);
        let en = part_2::check_lm1_moment(AnnexChoice::En, 20.0, 1, 3.0, 3000.0);
        assert!(de.computed.value < en.computed.value);
    }

    #[test]
    fn evaluate_reaches_every_part_module() {
        let report = evaluate(&Document::default());
        assert!(report.checks.iter().any(|c| c.clause.family.contains("1991-1-1")));
        assert!(report.checks.iter().any(|c| c.clause.family.contains("1991-1-2")));
        assert!(report.checks.iter().any(|c| c.clause.family.contains("1991-1-3")));
        assert!(report.checks.iter().any(|c| c.clause.family.contains("1991-1-4")));
        assert!(report.checks.iter().any(|c| c.clause.family.contains("1991-1-5")));
        assert!(report.checks.iter().any(|c| c.clause.family.contains("1991-1-6")));
        assert!(report.checks.iter().any(|c| c.clause.family.contains("1991-1-7")));
        assert!(report.checks.iter().any(|c| c.clause.family.contains("1991-2")));
        assert!(report.checks.iter().any(|c| c.clause.family.contains("1991-3")));
        assert!(report.checks.iter().any(|c| c.clause.family.contains("1991-4")));
    }
}


/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "en1991.document",
        extension: Some("en1991"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::en1990::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::en1990::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::en1990::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::en1990::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("en1991.document"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "en1991.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::en1990::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::en1990::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::en1990::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::en1990::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("en1991.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "en1991.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::en1990::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::en1990::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("en1991.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "en1991.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::en1990::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::en1990::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("en1991.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "en1991.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::en1990::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::en1990::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("en1991.spr"),
    });
}
