//! 🧱 EN 1996 design of masonry structures.

use norm_core::{AnnexChoice, CheckReport, CheckResult, ClauseId, Quantity};

pub mod na_de {
    pub use norm_en_1990::na_de::NaDe;
}

// #region 🔖Part1_1
pub mod part_1_1 {
    use super::*;

    pub fn flexural_resistance_knm(z_mm3: f64, f_xd_mpa: f64) -> f64 {
        z_mm3 * f_xd_mpa / 1_000_000.0
    }

    pub fn check_flexure(m_ed: f64, m_rd: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1996-1-1", "§6.2", "6.2"),
            Quantity::new(norm_core::QuantityKind::Moment, m_ed * 1_000_000.0),
            Quantity::new(norm_core::QuantityKind::Moment, m_rd * 1_000_000.0),
            "masonry flexure ULS",
            annex,
        )
    }

    pub fn check_compression(sigma_ed_mpa: f64, f_d_mpa: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1996-1-1", "§6.1.2", "6.1"),
            Quantity::stress_mpa(sigma_ed_mpa),
            Quantity::stress_mpa(f_d_mpa),
            "masonry compression ULS",
            annex,
        )
    }
}
// #endregion 🔖Part1_1

// #region 🔖Part1_2
pub mod part_1_2 {
    use super::*;

    pub fn check_fire_wall(thickness_mm: f64, required_mm: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1996-1-2", "§4", "4.1"),
            Quantity::length_m(thickness_mm / 1000.0),
            Quantity::length_m(required_mm / 1000.0),
            "masonry fire wall thickness",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖Part1_2

// #region 🔖Part2
pub mod part_2 {
    use super::*;

    pub fn check_lintel_shear(v_ed_kn: f64, v_rd_kn: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1996-2", "§6", "6.1"),
            Quantity::force_kn(v_ed_kn),
            Quantity::force_kn(v_rd_kn),
            "masonry lintel shear",
            AnnexChoice::En,
        )
    }
}
// #endregion 🔖Part2

// #region 🔖Part3
pub mod part_3 {
    use super::*;

    pub fn check_retaining_wall(m_ed: f64, m_rd: f64) -> CheckResult {
        part_1_1::check_flexure(m_ed, m_rd, AnnexChoice::En)
    }
}
// #endregion 🔖Part3

/// 📋 Masonry wall under vertical load.
pub fn check_masonry_wall(
    n_ed_kn: f64,
    area_mm2: f64,
    f_k_mpa: f64,
    gamma_m: f64,
) -> CheckReport {
    let sigma = n_ed_kn * 1000.0 / area_mm2;
    let f_d = f_k_mpa / gamma_m;
    let mut report = CheckReport::default();
    report.push(part_1_1::check_compression(sigma, f_d, AnnexChoice::De));
    report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn masonry_wall_e2e() {
        let report = check_masonry_wall(200.0, 500_000.0, 5.0, 2.0);
        assert!(!report.checks.is_empty());
    }
}
