//! 🌬️ EN 1991 actions on structures: imposed, wind, snow, thermal, cranes, accidental.

use norm_core::{AnnexChoice, CheckReport, CheckResult, ClauseId, NormError, Quantity};
use norm_en_1990::{na_de::NaDe, na_en::NaEn, NationalAnnex};

pub mod na_de {
    pub use norm_en_1990::na_de::NaDe;
}

// #region 🔖Part1_1
pub mod part_1_1 {
    use super::*;

    pub fn imposed_load_kn_m2(category: &str) -> f64 {
        match category {
            "A" => 1.75,
            "B" => 2.5,
            "C3" => 3.0,
            "D" => 4.0,
            _ => 2.0,
        }
    }

    pub fn check_imposed(area_m2: f64, category: &str, annex: &dyn NationalAnnex) -> CheckResult {
        let q = imposed_load_kn_m2(category) * area_m2;
        let psi = annex.psi_0(category);
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
    use super::*;

    pub fn self_weight_kn_m3(material: &str) -> f64 {
        match material {
            "concrete" => 25.0,
            "steel" => 78.5,
            "timber" => 5.0,
            _ => 20.0,
        }
    }
}
// #endregion 🔖Part1_2

// #region 🔖Part1_3
pub mod part_1_3 {
    use super::*;

    pub fn check_temperature_action(delta_t_k: f64, limit_k: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1991-1-3", "§6", "6.1"),
            Quantity::new(norm_core::QuantityKind::Temperature, delta_t_k),
            Quantity::new(norm_core::QuantityKind::Temperature, limit_k),
            "thermal action",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖Part1_3

// #region 🔖Part1_4
pub mod part_1_4 {
    use super::*;

    pub fn crane_vertical_wheel_load(crane_class: &str) -> f64 {
        match crane_class {
            "HC1" => 50.0,
            "HC2" => 100.0,
            _ => 80.0,
        }
    }
}
// #endregion 🔖Part1_4

// #region 🔖Part1_5
pub mod part_1_5 {
    use super::*;

    pub fn check_imposed_bridge(lane_load_kn: f64, design_kn: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1991-1-5", "§4", "4.3"),
            Quantity::force_kn(design_kn),
            Quantity::force_kn(lane_load_kn),
            "bridge imposed load",
            AnnexChoice::En,
        )
    }
}
// #endregion 🔖Part1_5

// #region 🔖Part1_6
pub mod part_1_6 {
    use super::*;

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

    /// 🌬️ Peak velocity pressure q_p [kN/m²] per EN 1991-1-4.
    pub fn peak_velocity_pressure(rho: f64, v_b_m_s: f64) -> f64 {
        0.5 * rho * v_b_m_s * v_b_m_s / 1000.0
    }

    pub fn wind_pressure(q_p: f64, c_pe: f64, c_pi: f64) -> f64 {
        q_p * (c_pe - c_pi)
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
// #endregion 🔖Part2

// #region 🔖Part3
pub mod part_3 {
    use super::*;

    /// ❄️ Ground snow load s_k [kN/m²] for zone (DE NA simplified).
    pub fn ground_snow_load_zone(zone: u8) -> f64 {
        match zone {
            1 => 0.65,
            2 => 0.85,
            3 => 1.1,
            _ => 0.85,
        }
    }

    pub fn roof_snow_load(s_k: f64, mu: f64) -> f64 {
        mu * s_k
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
// #endregion 🔖Part3

// #region 🔖Part4
pub mod part_4 {
    use super::*;

    pub fn thermal_coefficient_alpha_k_inv: f64 {
        1.0e-5
    }
}
// #endregion 🔖Part4

/// 📋 Aggregate action checks for a typical floor bay.
pub fn check_floor_actions(
    area_m2: f64,
    category: &str,
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
    let q_p = part_2::peak_velocity_pressure(1.25, wind_zone_vb);
    report.push(part_2::check_wind(part_2::wind_pressure(q_p, 0.8, 0.2), 1.5, annex));
    let s = part_3::roof_snow_load(part_3::ground_snow_load_zone(snow_zone), 0.8);
    report.push(part_3::check_snow(s, 1.2, annex));
    report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn floor_actions_de_na_e2e() {
        let report = check_floor_actions(50.0, "office", 25.0, 2, true);
        assert!(!report.checks.is_empty());
    }
}
