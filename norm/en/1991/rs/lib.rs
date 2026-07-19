//! 🌬️ EN 1991 actions on structures: imposed, snow, wind, thermal, cranes, accidental.

use norm_core::{AnnexChoice, CheckReport, CheckResult, ClauseId, ImposedCategory, Quantity};
use norm_en_1990::{na_de::NaDe, na_en::NaEn, NationalAnnex};

// #region 🔖NaDe
pub mod na_de {
    pub use norm_en_1990::na_de::NaDe;

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
// #endregion 🔖NaDe

// #region 🔖Part1_1
pub mod part_1_1 {
    use super::*;

    pub fn imposed_load_kn_m2(category: ImposedCategory) -> f64 {
        category.q_k_kn_m2()
    }

    pub fn check_imposed(area_m2: f64, category: ImposedCategory, annex: &dyn NationalAnnex) -> CheckResult {
        let q = imposed_load_kn_m2(category) * area_m2;
        let psi = annex.psi_0(category.label());
        CheckResult::from_utilization(
            ClauseId::new("EN 1991-1-1", "Table 6.1", "q"),
            Quantity::force_kn(q * psi),
            Quantity::force_kn(q),
            "imposed load",
            annex.choice(),
        )
    }
}
// #endregion 🔖Part1_1

// #region 🔖Part1_2
pub mod part_1_2 {
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
}
// #endregion 🔖Part1_2

// #region 🔖Part1_3
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

    pub fn check_snow(s_kn_m2: f64, limit: f64, annex: &dyn NationalAnnex) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1991-1-3", "§5", "5.1"),
            Quantity::new(norm_core::QuantityKind::Pressure, s_kn_m2 * 1000.0),
            Quantity::new(norm_core::QuantityKind::Pressure, limit * 1000.0),
            "snow load",
            annex.choice(),
        )
    }
}
// #endregion 🔖Part1_3

// #region 🔖Part1_4
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

    pub fn check_wind(w_p_kn_m2: f64, limit: f64, annex: &dyn NationalAnnex) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1991-1-4", "§5", "5.1"),
            Quantity::new(norm_core::QuantityKind::Pressure, w_p_kn_m2 * 1000.0),
            Quantity::new(norm_core::QuantityKind::Pressure, limit * 1000.0),
            "wind pressure",
            annex.choice(),
        )
    }
}
// #endregion 🔖Part1_4

// #region 🔖Part1_5
pub mod part_1_5 {
    use super::*;

    pub fn thermal_coefficient_alpha_k_inv() -> f64 {
        1.0e-5
    }

    pub fn temperature_difference_action(delta_t_k: f64, alpha: f64, e_modulus_gpa: f64) -> f64 {
        alpha * delta_t_k * e_modulus_gpa
    }

    pub fn check_temperature_action(delta_t_k: f64, limit_k: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1991-1-5", "§6", "6.1"),
            Quantity::new(norm_core::QuantityKind::Temperature, delta_t_k),
            Quantity::new(norm_core::QuantityKind::Temperature, limit_k),
            "thermal action",
            AnnexChoice::De,
        )
    }

    pub fn check_fire_boundary_temperature(t_surface_k: f64, t_limit_k: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1991-1-5", "Annex A", "A.1"),
            Quantity::new(norm_core::QuantityKind::Temperature, t_surface_k),
            Quantity::new(norm_core::QuantityKind::Temperature, t_limit_k),
            "fire boundary temperature",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖Part1_5

// #region 🔖Part1_6
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
        CheckResult::from_utilization(
            ClauseId::new("EN 1991-1-6", "§4", "4.1"),
            Quantity::force_kn(q_kn_m2),
            Quantity::force_kn(limit),
            "construction load",
            AnnexChoice::En,
        )
    }
}
// #endregion 🔖Part1_6

// #region 🔖Part1_7
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
            Quantity::new(norm_core::QuantityKind::Pressure, p_kpa * 1000.0),
            Quantity::new(norm_core::QuantityKind::Pressure, limit_kpa * 1000.0),
            "accidental pressure",
            AnnexChoice::En,
        )
    }
}
// #endregion 🔖Part1_7

// #region 🔖Part2
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

    pub fn check_imposed_bridge(lane_load_kn: f64, design_kn: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1991-2", "§4", "4.3"),
            Quantity::force_kn(design_kn),
            Quantity::force_kn(lane_load_kn),
            "bridge imposed load",
            AnnexChoice::En,
        )
    }
}
// #endregion 🔖Part2

// #region 🔖Part3
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

    pub fn check_crane_load(wheel_load_kn: f64, capacity_kn: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1991-3", "§2", "2.3"),
            Quantity::force_kn(wheel_load_kn),
            Quantity::force_kn(capacity_kn),
            "crane wheel load",
            AnnexChoice::En,
        )
    }
}
// #endregion 🔖Part3

// #region 🔖Part4
pub mod part_4 {
    use super::*;

    pub fn silo_wall_pressure_kpa(bulk_density_kn_m3: f64, height_m: f64, k: f64) -> f64 {
        k * bulk_density_kn_m3 * height_m
    }

    pub fn tank_hydrostatic_pressure_kpa(fluid_density_kn_m3: f64, fill_height_m: f64) -> f64 {
        fluid_density_kn_m3 * fill_height_m
    }

    pub fn check_silo_pressure(p_kpa: f64, limit_kpa: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1991-4", "§5", "5.1"),
            Quantity::new(norm_core::QuantityKind::Pressure, p_kpa * 1000.0),
            Quantity::new(norm_core::QuantityKind::Pressure, limit_kpa * 1000.0),
            "silo wall pressure",
            AnnexChoice::En,
        )
    }
}
// #endregion 🔖Part4

/// 📋 Aggregate action checks for a typical floor bay.
pub fn check_floor_actions(
    area_m2: f64,
    category: ImposedCategory,
    wind_zone_vb: f64,
    snow_zone: u8,
    use_de_na: bool,
) -> CheckReport {
    let annex: &dyn NationalAnnex = if use_de_na {
        &NaDe
    } else {
        &NaEn
    };
    let mut report = CheckReport::default();
    report.push(part_1_1::check_imposed(area_m2, category, annex));
    let c_e = part_1_4::exposure_factor(10.0, part_1_4::TerrainCategory::II);
    let q_p = part_1_4::peak_velocity_pressure(1.25, wind_zone_vb, c_e);
    report.push(part_1_4::check_wind(
        part_1_4::wind_pressure(q_p, 0.8, 0.2),
        1.5,
        annex,
    ));
    let s = part_1_3::roof_snow_load(part_1_3::ground_snow_load_zone(snow_zone), 0.8);
    report.push(part_1_3::check_snow(s, 1.2, annex));
    report
}

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
    fn floor_actions_de_na_e2e() {
        let report = check_floor_actions(50.0, ImposedCategory::B, 25.0, 2, true);
        assert!(!report.checks.is_empty());
    }

    #[test]
    fn de_wind_zone_2_basic_velocity() {
        assert!((na_de::WindZone::Zone2.v_b_m_s() - 25.0).abs() < 1e-9);
    }
}
