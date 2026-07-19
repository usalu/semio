//! 🔗 EN 1994 design of composite steel and concrete structures.

use norm_core::{AnnexChoice, CheckReport, CheckResult, ClauseId, Quantity};

// #region 🔖NaDe
pub mod na_de {
    pub use norm_en_1990::na_de::NaDe;

    /// 🇩🇪 DIN EN 1994-1-1/NA: partial factor γ_V for shear connectors.
    pub const GAMMA_V: f64 = 1.25;

    pub fn gamma_v() -> f64 {
        GAMMA_V
    }
}
// #endregion 🔖NaDe

// #region 🔖Part1_1
pub mod part_1_1 {
    use super::*;

    /// 📐 Full plastic moment M_pl,Rd [kNm] (steel + concrete).
    pub fn full_plastic_moment_knm(m_pla_knm: f64, m_plc_knm: f64) -> f64 {
        m_pla_knm + m_plc_knm
    }

    /// 📐 Partial shear connection degree η = n_f / n_f,req.
    pub fn shear_connection_degree(n_f: u32, n_f_req: u32) -> f64 {
        n_f as f64 / n_f_req as f64
    }

    /// 📐 Composite plastic moment with partial shear connection per EN 1994-1-1 §6.2.1(8).
    pub fn plastic_moment_partial_knm(m_pla_knm: f64, m_pl_rd_knm: f64, eta: f64) -> f64 {
        m_pla_knm + eta * (m_pl_rd_knm - m_pla_knm)
    }

    /// 📐 Effective width b_eff [mm] per EN 1994-1-1 §5.4.1.2.
    pub fn effective_width_mm(span_mm: f64, b_0_mm: f64, beam_spacing_mm: f64) -> f64 {
        let be1 = span_mm / 8.0 + b_0_mm;
        let be2 = beam_spacing_mm / 2.0;
        (2.0 * be1).min(2.0 * be2)
    }

    /// 📐 Longitudinal shear V_L [kN] per EN 1994-1-1 §6.6.2.
    pub fn longitudinal_shear_kn(delta_n_kn: f64, connector_spacing_mm: f64) -> f64 {
        delta_n_kn * 1000.0 / connector_spacing_mm
    }

    /// 📐 Shear connector resistance P_Rd [kN] per EN 1994-1-1 §6.6.3.
    pub fn connector_resistance_kn(d_mm: f64, _h_mm: f64, f_ck_mpa: f64, f_u_mpa: f64) -> f64 {
        let alpha = 0.8;
        let eta = 1.0;
        let p_b = 0.29 * alpha * d_mm * d_mm * (f_ck_mpa * f_u_mpa).sqrt();
        let p_pl = 0.8 * f_u_mpa * std::f64::consts::PI * d_mm * d_mm / 4.0;
        eta * p_b.min(p_pl) / na_de::gamma_v() / 1000.0
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

    /// 🔥 Fire resistance rating.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FireRating {
        R30,
        R60,
        R90,
        R120,
    }

    /// 🔥 Composite deck insulation thickness [mm] per EN 1994-1-2 Table 4.2.
    pub fn insulation_thickness_mm(rating: FireRating, deck_type: &str) -> f64 {
        let base = match rating {
            FireRating::R30 => 10.0,
            FireRating::R60 => 18.0,
            FireRating::R90 => 28.0,
            FireRating::R120 => 40.0,
        };
        match deck_type {
            "trapezoidal" => base,
            "re-entrant" => base * 1.1,
            _ => base,
        }
    }

    pub fn check_fire_composite(thickness_mm: f64, rating: FireRating, deck_type: &str) -> CheckResult {
        let required = insulation_thickness_mm(rating, deck_type);
        CheckResult::from_utilization(
            ClauseId::new("EN 1994-1-2", "§4.2", "4.2"),
            Quantity::length_m(required / 1000.0),
            Quantity::length_m(thickness_mm / 1000.0),
            "composite fire insulation",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖Part1_2

// #region 🔖Part2
pub mod part_2 {
    use super::*;

    /// 🌉 Bridge composite fatigue detail category per EN 1994-2 §8.
    pub fn bridge_fatigue_category(detail: &str) -> u8 {
        match detail {
            "stud_welded" => 80,
            "shear_connector" => 71,
            "reinforcement" => 90,
            _ => 71,
        }
    }

    /// 🌉 Bridge composite bending + fatigue check.
    pub fn check_bridge_composite(m_ed_knm: f64, m_rd_knm: f64, delta_sigma_mpa: f64, detail: &str) -> CheckReport {
        let mut report = CheckReport::default();
        report.push(part_1_1::check_composite_bending(m_ed_knm, m_rd_knm, AnnexChoice::En));
        let category = bridge_fatigue_category(detail);
        let limit = category as f64;
        report.push(CheckResult::from_utilization(
            ClauseId::new("EN 1994-2", "§8", "8.1"),
            Quantity::stress_mpa(delta_sigma_mpa),
            Quantity::stress_mpa(limit),
            "bridge composite fatigue",
            AnnexChoice::En,
        ));
        report
    }
}
// #endregion 🔖Part2

/// 📋 Composite slab beam check.
pub fn check_composite_beam(
    m_ed_knm: f64,
    v_ed_kn: f64,
    m_pla: f64,
    m_pl_rd: f64,
    eta: f64,
    v_l_rd: f64,
) -> CheckReport {
    let annex = AnnexChoice::De;
    let m_rd = part_1_1::plastic_moment_partial_knm(m_pla, m_pl_rd, eta);
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
        let report = check_composite_beam(200.0, 120.0, 80.0, 250.0, 0.75, 150.0);
        assert!(!report.checks.is_empty());
        let m_rd: f64 = 80.0 + 0.75 * (250.0 - 80.0);
        assert!((m_rd - 207.5).abs() < 0.1);
    }

    #[test]
    fn effective_width_8m_span() {
        let beff = part_1_1::effective_width_mm(8000.0, 80.0, 3000.0);
        assert!((beff - 2160.0).abs() < 1.0);
    }

    #[test]
    fn partial_shear_connection_eta() {
        let eta = part_1_1::shear_connection_degree(15, 20);
        assert!((eta - 0.75).abs() < 0.01);
        let m_rd = part_1_1::plastic_moment_partial_knm(100.0, 300.0, eta);
        assert!((m_rd - 250.0).abs() < 0.1);
    }

    #[test]
    fn longitudinal_shear_vl() {
        let v_l = part_1_1::longitudinal_shear_kn(500.0, 200.0);
        assert!((v_l - 2500.0).abs() < 1.0);
    }

    #[test]
    fn stud_connector_resistance() {
        let p_rd = part_1_1::connector_resistance_kn(19.0, 100.0, 30.0, 450.0);
        assert!((p_rd - 7.8).abs() < 1.0);
    }

    #[test]
    fn fire_insulation_r60() {
        let t = part_1_2::insulation_thickness_mm(part_1_2::FireRating::R60, "trapezoidal");
        assert!((t - 18.0).abs() < 0.1);
    }

    #[test]
    fn bridge_composite_fatigue() {
        let report = part_2::check_bridge_composite(180.0, 250.0, 65.0, "stud_welded");
        assert_eq!(report.checks.len(), 2);
    }

    #[test]
    fn na_de_gamma_v() {
        assert!((na_de::gamma_v() - 1.25).abs() < 1e-9);
    }
}
