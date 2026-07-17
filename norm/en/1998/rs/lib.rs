//! 🌋 EN 1998 design of structures for earthquake resistance.

use norm_core::{AnnexChoice, CheckReport, CheckResult, ClauseId, Quantity};

pub mod na_de {
    pub use norm_en_1990::na_de::NaDe;
}

// #region 🔖Part1
pub mod part_1 {
    use super::*;

    pub fn design_spectrum_accel(a_g: f64, s: f64, tb: f64, tc: f64, td: f64, t: f64) -> f64 {
        if t <= tb {
            a_g * s * (1.0 + t / tb * (2.5 / s - 1.0))
        } else if t <= tc {
            a_g * s * 2.5 / s
        } else if t <= td {
            a_g * s * 2.5 / s * tc / t
        } else {
            a_g * s * 2.5 / s * tc * td / (t * t)
        }
    }

    pub fn check_drift(drift_mm: f64, limit_mm: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1998-1", "§4.3", "4.3.3"),
            Quantity::length_m(drift_mm / 1000.0),
            Quantity::length_m(limit_mm / 1000.0),
            "interstorey drift SLS",
            annex,
        )
    }
}
// #endregion 🔖Part1

// #region 🔖Part2
pub mod part_2 {
    use super::*;

    pub fn check_bridge_seismic(v_ed: f64, v_rd: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1998-2", "§5", "5.3"),
            Quantity::force_kn(v_ed),
            Quantity::force_kn(v_rd),
            "bridge seismic shear",
            AnnexChoice::En,
        )
    }
}
// #endregion 🔖Part2

// #region 🔖Part3
pub mod part_3 {
    use super::part_1;

    pub fn check_silos(drift: f64, limit: f64) -> CheckResult {
        part_1::check_drift(drift, limit, AnnexChoice::En)
    }
}
// #endregion 🔖Part3

// #region 🔖Part4
pub mod part_4 {
    use super::*;
}
// #endregion 🔖Part4

// #region 🔖Part5
pub mod part_5 {
    use super::*;
}
// #endregion 🔖Part5

// #region 🔖Part6
pub mod part_6 {
    use super::*;
}
// #endregion 🔖Part6

/// 📋 Building seismic check (DE NA zone parameters).
pub fn check_building_seismic(
    a_g: f64,
    importance_gamma_i: f64,
    q_ed_kn: f64,
    q_rd_kn: f64,
    drift_mm: f64,
    height_m: f64,
) -> CheckReport {
    let annex = AnnexChoice::De;
    let s = design_spectrum_factor(a_g);
    let _ = part_1::design_spectrum_accel(a_g, s, 0.15, 0.5, 2.0, 0.3);
    let drift_limit = 0.005 * height_m * 1000.0;
    let mut report = CheckReport::default();
    report.push(CheckResult::from_utilization(
        ClauseId::new("EN 1998-1", "§4.3", "4.3.4"),
        Quantity::force_kn(q_ed_kn * importance_gamma_i),
        Quantity::force_kn(q_rd_kn),
        "seismic base shear ULS",
        annex,
    ));
    report.push(part_1::check_drift(drift_mm, drift_limit, annex));
    report
}

fn design_spectrum_factor(a_g: f64) -> f64 {
    if a_g <= 0.1 {
        1.0
    } else if a_g <= 0.2 {
        1.2
    } else {
        1.4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn building_seismic_e2e() {
        let report = check_building_seismic(0.15, 1.2, 400.0, 600.0, 20.0, 12.0);
        assert!(!report.checks.is_empty());
    }
}
