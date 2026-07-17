//! 🔗 EN 1994 design of composite steel and concrete structures.

use norm_core::{AnnexChoice, CheckReport, CheckResult, ClauseId, Quantity};

pub mod na_de {
    pub use norm_en_1990::na_de::NaDe;
}

// #region 🔖Part1_1
pub mod part_1_1 {
    use super::*;

    pub fn plastic_moment_knm(m_pla_knm: f64, m_pr_knm: f64, m_a_knm: f64) -> f64 {
        m_pla_knm + m_pr_knm + m_a_knm
    }

    pub fn check_composite_bending(m_ed: f64, m_rd: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1994-1-1", "§6.2", "6.2"),
            Quantity::new(norm_core::QuantityKind::Moment, m_ed * 1_000_000.0),
            Quantity::new(norm_core::QuantityKind::Moment, m_rd * 1_000_000.0),
            "composite bending ULS",
            annex,
        )
    }

    pub fn check_longitudinal_shear(v_ed_kn: f64, v_l_rd_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1994-1-1", "§6.6", "6.6"),
            Quantity::force_kn(v_ed_kn),
            Quantity::force_kn(v_l_rd_kn),
            "longitudinal shear",
            annex,
        )
    }
}
// #endregion 🔖Part1_1

// #region 🔖Part1_2
pub mod part_1_2 {
    use super::*;

    pub fn check_fire_composite(cover_mm: f64, required_mm: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1994-1-2", "§4", "4.2"),
            Quantity::length_m(cover_mm / 1000.0),
            Quantity::length_m(required_mm / 1000.0),
            "composite fire protection",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖Part1_2

// #region 🔖Part2
pub mod part_2 {
    use super::part_1_1;

    pub fn check_bridge_composite(m_ed: f64, m_rd: f64) -> CheckResult {
        part_1_1::check_composite_bending(m_ed, m_rd, AnnexChoice::En)
    }
}
// #endregion 🔖Part2

/// 📋 Composite slab beam check.
pub fn check_composite_beam(
    m_ed_knm: f64,
    v_ed_kn: f64,
    m_pla: f64,
    m_pr: f64,
    m_a: f64,
    v_l_rd: f64,
) -> CheckReport {
    let annex = AnnexChoice::De;
    let m_rd = part_1_1::plastic_moment_knm(m_pla, m_pr, m_a);
    let mut report = CheckReport::default();
    report.push(part_1_1::check_composite_bending(m_ed_knm, m_rd, annex));
    report.push(part_1_1::check_longitudinal_shear(v_ed_kn, v_l_rd, annex));
    report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composite_beam_e2e() {
        let report = check_composite_beam(200.0, 120.0, 80.0, 100.0, 50.0, 150.0);
        assert!(!report.checks.is_empty());
    }
}
