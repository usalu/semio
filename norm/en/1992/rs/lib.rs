//! 🧱 EN 1992 design of concrete structures.

use norm_core::{AnnexChoice, CheckReport, CheckResult, ClauseId, NormError, Quantity};
use norm_en_1990::{na_de::NaDe, NationalAnnex};
use norm_en_1991::part_1_1;

pub mod na_de {
    pub use norm_en_1990::na_de::NaDe;
}

// #region 🔖Part1_1
pub mod part_1_1 {
    use super::*;

    /// 📐 Flexural resistance M_Rd [kNm] per EN 1992-1-1 §6.1.
    pub fn flexural_resistance_knm(f_ck: f64, b_mm: f64, d_mm: f64, a_s_mm2: f64, f_yk: f64) -> f64 {
        let f_cd = 0.85 * f_ck / 1.5 / 1000.0;
        let f_yd = f_yk / 1.15 / 1000.0;
        let x = a_s_mm2 * f_yd / (0.8 * b_mm * f_cd);
        let z = d_mm - 0.4 * x;
        a_s_mm2 * f_yd * z / 1_000_000.0
    }

    /// 📐 Shear resistance V_Rd,c [kN] per EN 1992-1-1 §6.2.2.
    pub fn shear_resistance_vrdc_kn(b_mm: f64, d_mm: f64, f_ck: f64, rho_l: f64, n_ed_kn: f64) -> f64 {
        let k = (200.0 / d_mm).min(2.0).sqrt();
        let sigma_cp = (n_ed_kn * 1000.0 / (b_mm * d_mm)).max(0.0);
        let v_min = 0.035 * k.powf(1.5) * f_ck.sqrt();
        let v_rd = (0.18 / 1.5) * k * (100.0 * rho_l * f_ck).sqrt() + 0.15 * sigma_cp;
        v_rd.max(v_min) * b_mm * d_mm / 1000.0
    }

    pub fn check_flexure(m_ed_knm: f64, m_rd_knm: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1992-1-1", "§6.1", "6.1"),
            Quantity::new(norm_core::QuantityKind::Moment, m_ed_knm * 1_000_000.0),
            Quantity::new(norm_core::QuantityKind::Moment, m_rd_knm * 1_000_000.0),
            "flexural ULS",
            annex,
        )
    }

    pub fn check_shear(v_ed_kn: f64, v_rd_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1992-1-1", "§6.2", "6.2"),
            Quantity::force_kn(v_ed_kn),
            Quantity::force_kn(v_rd_kn),
            "shear ULS",
            annex,
        )
    }
}
// #endregion 🔖Part1_1

// #region 🔖Part1_2
pub mod part_1_2 {
    use super::*;

    pub fn check_fire_cover(cover_mm: f64, required_mm: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1992-1-2", "§4", "4.2"),
            Quantity::length_m(cover_mm / 1000.0),
            Quantity::length_m(required_mm / 1000.0),
            "fire cover",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖Part1_2

// #region 🔖Part2
pub mod part_2 {
    use super::part_1_1;

    pub fn check_bridge_flexure(m_ed: f64, m_rd: f64) -> CheckResult {
        part_1_1::check_flexure(m_ed, m_rd, AnnexChoice::En)
    }
}
// #endregion 🔖Part2

// #region 🔖Part3
pub mod part_3 {
    use super::*;

    pub fn crack_width_wk_mm(eps_sm: f64, s_r_max_mm: f64) -> f64 {
        eps_sm * s_r_max_mm
    }

    pub fn check_crack_width(w_k: f64, limit: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1992-3", "§7", "7.1"),
            Quantity::length_m(w_k / 1000.0),
            Quantity::length_m(limit / 1000.0),
            "crack width SLS",
            AnnexChoice::En,
        )
    }
}
// #endregion 🔖Part3

// #region 🔖Part4
pub mod part_4 {
    use super::part_1_1;

    pub fn check_precast_flexure(m_ed: f64, m_rd: f64) -> CheckResult {
        part_1_1::check_flexure(m_ed, m_rd, AnnexChoice::De)
    }
}
// #endregion 🔖Part4

/// 📋 RC beam ULS check end-to-end.
pub fn check_rc_beam(
    m_ed_knm: f64,
    v_ed_kn: f64,
    f_ck: f64,
    b_mm: f64,
    d_mm: f64,
    a_s_mm2: f64,
    f_yk: f64,
    rho_l: f64,
    n_ed_kn: f64,
) -> CheckReport {
    let annex = AnnexChoice::De;
    let m_rd = part_1_1::flexural_resistance_knm(f_ck, b_mm, d_mm, a_s_mm2, f_yk);
    let v_rd = part_1_1::shear_resistance_vrdc_kn(b_mm, d_mm, f_ck, rho_l, n_ed_kn);
    let mut report = CheckReport::default();
    report.push(part_1_1::check_flexure(m_ed_knm, m_rd, annex));
    report.push(part_1_1::check_shear(v_ed_kn, v_rd, annex));
    report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rc_beam_e2e() {
        let report = check_rc_beam(120.0, 80.0, 30.0, 300.0, 500.0, 2500.0, 500.0, 0.01, 200.0);
        assert!(!report.checks.is_empty());
    }
}
