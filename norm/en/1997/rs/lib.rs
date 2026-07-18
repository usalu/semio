//! 🌍 EN 1997 geotechnical design.

use norm_core::{AnnexChoice, CheckReport, CheckResult, ClauseId, Quantity};

pub mod na_de {
    pub use norm_en_1990::na_de::NaDe;
}

// #region 🔖Part1
pub mod part_1 {
    use super::*;

    pub fn bearing_resistance_kn(a_m2: f64, q_ult_kpa: f64, gamma_r: f64) -> f64 {
        a_m2 * q_ult_kpa / gamma_r
    }

    pub fn check_bearing(v_ed_kn: f64, r_d_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1997-1", "§6.5", "6.5"),
            Quantity::force_kn(v_ed_kn),
            Quantity::force_kn(r_d_kn),
            "bearing resistance ULS",
            annex,
        )
    }

    pub fn check_sliding(h_ed_kn: f64, r_d_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1997-1", "§6.5", "6.5.3"),
            Quantity::force_kn(h_ed_kn),
            Quantity::force_kn(r_d_kn),
            "sliding resistance ULS",
            annex,
        )
    }

    pub fn check_settlement(s_mm: f64, limit_mm: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1997-1", "§6.6", "6.6"),
            Quantity::length_m(s_mm / 1000.0),
            Quantity::length_m(limit_mm / 1000.0),
            "settlement SLS",
            annex,
        )
    }
}
// #endregion 🔖Part1

// #region 🔖Part2
pub mod part_2 {
    use super::{part_1, AnnexChoice, CheckResult};

    pub fn check_pile_axial(n_ed_kn: f64, r_c_d_kn: f64) -> CheckResult {
        part_1::check_bearing(n_ed_kn, r_c_d_kn, AnnexChoice::De)
    }
}
// #endregion 🔖Part2

/// 📋 Shallow foundation check.
pub fn check_shallow_foundation(
    v_ed_kn: f64,
    h_ed_kn: f64,
    footing_area_m2: f64,
    q_ult_kpa: f64,
    mu: f64,
    n_kn: f64,
    settlement_mm: f64,
) -> CheckReport {
    let annex = AnnexChoice::De;
    let r_d = part_1::bearing_resistance_kn(footing_area_m2, q_ult_kpa, 1.4);
    let r_sliding = mu * n_kn;
    let mut report = CheckReport::default();
    report.push(part_1::check_bearing(v_ed_kn, r_d, annex));
    report.push(part_1::check_sliding(h_ed_kn, r_sliding, annex));
    report.push(part_1::check_settlement(settlement_mm, 25.0, annex));
    report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shallow_foundation_e2e() {
        let report = check_shallow_foundation(500.0, 80.0, 2.0, 300.0, 0.45, 500.0, 15.0);
        assert!(!report.checks.is_empty());
    }
}
