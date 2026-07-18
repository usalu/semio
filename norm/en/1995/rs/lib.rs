//! 🪵 EN 1995 design of timber structures.

use norm_core::{AnnexChoice, CheckReport, CheckResult, ClauseId, LoadDuration, Quantity};

pub mod na_de {
    pub use norm_en_1990::na_de::NaDe;
}

// #region 🔖Kmod
fn k_mod(duration: LoadDuration) -> f64 {
    match duration {
        LoadDuration::Permanent => 0.6,
        LoadDuration::Long => 0.7,
        LoadDuration::Medium => 0.8,
        LoadDuration::Short => 0.9,
        LoadDuration::Instantaneous => 1.1,
    }
}
// #endregion 🔖Kmod

// #region 🔖Part1_1
pub mod part_1_1 {
    use super::*;

    pub fn bending_resistance_knm(w_mm3: f64, f_m_k_mpa: f64, k_mod: f64) -> f64 {
        k_mod * w_mm3 * f_m_k_mpa / 1.3 / 1_000_000.0
    }

    pub fn check_bending(m_ed: f64, m_rd: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1995-1-1", "§6.1.6", "6.1"),
            Quantity::new(norm_core::QuantityKind::Moment, m_ed * 1_000_000.0),
            Quantity::new(norm_core::QuantityKind::Moment, m_rd * 1_000_000.0),
            "timber bending ULS",
            annex,
        )
    }

    pub fn check_compression(n_ed_kn: f64, n_rd_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1995-1-1", "§6.1.4", "6.1"),
            Quantity::force_kn(n_ed_kn),
            Quantity::force_kn(n_rd_kn),
            "timber compression ULS",
            annex,
        )
    }
}
// #endregion 🔖Part1_1

// #region 🔖Part1_2
pub mod part_1_2 {
    use super::*;

    pub fn check_fire(charred_depth_mm: f64, remaining_mm: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1995-1-2", "§4", "4.2"),
            Quantity::length_m(remaining_mm / 1000.0),
            Quantity::length_m(charred_depth_mm / 1000.0),
            "timber fire residual section",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖Part1_2

// #region 🔖Part2
pub mod part_2 {
    use super::{part_1_1, AnnexChoice, CheckResult};

    pub fn check_bridge_timber(m_ed: f64, m_rd: f64) -> CheckResult {
        part_1_1::check_bending(m_ed, m_rd, AnnexChoice::En)
    }
}
// #endregion 🔖Part2

/// 📋 Glulam beam check.
pub fn check_glulam_beam(
    m_ed_knm: f64,
    n_ed_kn: f64,
    w_mm3: f64,
    a_mm2: f64,
    f_m_k: f64,
    f_c_0_k: f64,
    duration: LoadDuration,
) -> CheckReport {
    let km = k_mod(duration);
    let m_rd = part_1_1::bending_resistance_knm(w_mm3, f_m_k, km);
    let n_rd = km * a_mm2 * f_c_0_k / 1.3 / 1000.0;
    let annex = AnnexChoice::De;
    let mut report = CheckReport::default();
    report.push(part_1_1::check_bending(m_ed_knm, m_rd, annex));
    report.push(part_1_1::check_compression(n_ed_kn, n_rd, annex));
    report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn glulam_beam_e2e() {
        let report = check_glulam_beam(25.0, 50.0, 1_000_000.0, 20_000.0, 24.0, 21.0, LoadDuration::Medium);
        assert!(!report.checks.is_empty());
    }
}
