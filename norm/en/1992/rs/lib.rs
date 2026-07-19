//! 🧱 EN 1992 design of concrete structures.

use norm_core::{AnnexChoice, CheckReport, CheckResult, ClauseId, Quantity};

// #region 🔖NaDe
pub mod na_de {
    pub use norm_en_1990::na_de::NaDe;

    /// 🇩🇪 DIN EN 1992-1-1/NA: long-term concrete strength factor.
    pub const ALPHA_CC: f64 = 0.85;

    pub fn alpha_cc() -> f64 {
        ALPHA_CC
    }
}
// #endregion 🔖NaDe

// #region 🔖Part1_1
pub mod part_1_1 {
    use super::*;

    /// 📐 Flexural resistance M_Rd [kNm] per EN 1992-1-1 §6.1.
    pub fn flexural_resistance_knm(f_ck: f64, b_mm: f64, d_mm: f64, a_s_mm2: f64, f_yk: f64) -> f64 {
        let f_cd = na_de::alpha_cc() * f_ck / 1.5 / 1000.0;
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

    /// 🕳️ Punching shear strength v_Rd,max [MPa] per EN 1992-1-1 Eq. 6.50.
    pub fn punching_v_rd_max_mpa(f_ck: f64) -> f64 {
        let f_cd = na_de::alpha_cc() * f_ck / 1.5;
        let nu = 0.6 * (1.0 - f_ck / 250.0);
        0.5 * nu * f_cd
    }

    /// 🕳️ Punching shear resistance V_Rd,max [kN] around perimeter u_1.
    pub fn punching_resistance_kn(f_ck: f64, u_1_mm: f64, d_mm: f64) -> f64 {
        punching_v_rd_max_mpa(f_ck) * u_1_mm * d_mm / 1000.0
    }

    /// 🔁 Torsional resistance T_Rd [kNm] per EN 1992-1-1 §6.3.2 (thin-walled hollow section).
    pub fn torsion_resistance_knm(f_ck: f64, a_k_mm2: f64, t_mm: f64) -> f64 {
        let f_cd = na_de::alpha_cc() * f_ck / 1.5 / 1000.0;
        let nu = 0.6 * (1.0 - f_ck / 250.0);
        let alpha_cw = 1.0;
        2.0 * nu * alpha_cw * f_cd * t_mm * a_k_mm2 / 1_000_000.0
    }

    /// 📏 Slenderness λ = l_0 / i.
    pub fn slenderness_lambda(l_0_mm: f64, i_mm: f64) -> f64 {
        l_0_mm / i_mm
    }

    /// 📏 Radius of gyration i [mm] from area and second moment.
    pub fn radius_of_gyration_mm(a_mm2: f64, i_mm4: f64) -> f64 {
        (i_mm4 / a_mm2).sqrt()
    }

    /// 🪟 Crack width w_k [mm] per EN 1992-1-1 Eq. 7.8.
    pub fn crack_width_wk_mm(eps_sm: f64, eps_cm: f64, s_r_max_mm: f64) -> f64 {
        (eps_sm - eps_cm).max(0.0) * s_r_max_mm
    }

    /// 🪟 Mean steel strain ε_sm per EN 1992-1-1 Eq. 7.9.
    pub fn steel_strain_eps_sm(sigma_s_mpa: f64, rho_p_eff: f64, f_ct_eff_mpa: f64, e_s_mpa: f64) -> f64 {
        let term = (f_ct_eff_mpa / rho_p_eff / e_s_mpa).max(0.6 * sigma_s_mpa / e_s_mpa);
        (sigma_s_mpa / e_s_mpa) * (1.0 - term).max(0.4)
    }

    /// 📉 Immediate deflection δ [mm] of simply supported beam under UDL.
    pub fn deflection_ss_udl_mm(w_kn_m: f64, span_m: f64, e_mpa: f64, i_mm4: f64) -> f64 {
        let w = w_kn_m;
        let l = span_m * 1000.0;
        5.0 * w * l.powi(4) / (384.0 * e_mpa * i_mm4)
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

    pub fn check_punching(v_ed_kn: f64, v_rd_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1992-1-1", "§6.4", "6.4"),
            Quantity::force_kn(v_ed_kn),
            Quantity::force_kn(v_rd_kn),
            "punching shear ULS",
            annex,
        )
    }

    pub fn check_torsion(t_ed_knm: f64, t_rd_knm: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1992-1-1", "§6.3", "6.3"),
            Quantity::new(norm_core::QuantityKind::Moment, t_ed_knm * 1_000_000.0),
            Quantity::new(norm_core::QuantityKind::Moment, t_rd_knm * 1_000_000.0),
            "torsion ULS",
            annex,
        )
    }

    pub fn check_crack_width(w_k_mm: f64, limit_mm: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1992-1-1", "§7.3", "7.3"),
            Quantity::length_m(w_k_mm / 1000.0),
            Quantity::length_m(limit_mm / 1000.0),
            "crack width SLS",
            annex,
        )
    }

    pub fn check_deflection(delta_mm: f64, limit_mm: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1992-1-1", "§7.4", "7.4"),
            Quantity::length_m(delta_mm / 1000.0),
            Quantity::length_m(limit_mm / 1000.0),
            "deflection SLS",
            annex,
        )
    }
}
// #endregion 🔖Part1_1

// #region 🔖Part1_2
pub mod part_1_2 {
    use super::*;

    /// 🏗️ Fire resistance rating.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FireRating {
        R30,
        R60,
        R90,
    }

    /// 🏗️ Structural element type for fire cover lookup.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ElementType {
        Slab,
        Beam,
        Column,
    }

    /// 🔥 Minimum axis distance a_min [mm] per EN 1992-1-2 Table 5.5 (simplified tabulated values).
    pub fn min_axis_distance_mm(element: ElementType, rating: FireRating) -> f64 {
        match (element, rating) {
            (ElementType::Slab, FireRating::R30) => 10.0,
            (ElementType::Slab, FireRating::R60) => 20.0,
            (ElementType::Slab, FireRating::R90) => 30.0,
            (ElementType::Beam, FireRating::R30) => 25.0,
            (ElementType::Beam, FireRating::R60) => 35.0,
            (ElementType::Beam, FireRating::R90) => 50.0,
            (ElementType::Column, FireRating::R30) => 25.0,
            (ElementType::Column, FireRating::R60) => 40.0,
            (ElementType::Column, FireRating::R90) => 55.0,
        }
    }

    pub fn check_fire_cover(cover_mm: f64, element: ElementType, rating: FireRating) -> CheckResult {
        let required = min_axis_distance_mm(element, rating);
        CheckResult::from_utilization(
            ClauseId::new("EN 1992-1-2", "§4.2", "4.2"),
            Quantity::length_m(required / 1000.0),
            Quantity::length_m(cover_mm / 1000.0),
            "fire axis distance",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖Part1_2

// #region 🔖Part2
pub mod part_2 {
    use super::{part_1_1, AnnexChoice, CheckResult};

    pub fn check_bridge_flexure(m_ed: f64, m_rd: f64) -> CheckResult {
        part_1_1::check_flexure(m_ed, m_rd, AnnexChoice::En)
    }
}
// #endregion 🔖Part2

// #region 🔖Part3
pub mod part_3 {
    use super::*;

    /// 💧 Exposure class steel stress limit σ_s,lim [MPa] per EN 1992-3 Table 7.1N.
    pub fn steel_stress_limit_mpa(exposure: &str) -> f64 {
        match exposure {
            "XC1" | "XC2" => 250.0,
            "XC3" | "XC4" => 200.0,
            "XD1" | "XD2" | "XD3" => 160.0,
            "XS1" | "XS2" | "XS3" => 160.0,
            _ => 200.0,
        }
    }

    /// 🪟 Liquid-retaining crack width w_k [mm] with steel stress limit per EN 1992-3 §7.
    pub fn crack_width_liquid_mm(sigma_s_mpa: f64, exposure: &str, s_r_max_mm: f64, e_s_mpa: f64) -> f64 {
        let limit = steel_stress_limit_mpa(exposure);
        let sigma_eff = sigma_s_mpa.min(limit);
        let eps_sm = sigma_eff / e_s_mpa;
        eps_sm * s_r_max_mm
    }

    pub fn check_liquid_crack_width(w_k: f64, limit: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1992-3", "§7", "7.1"),
            Quantity::length_m(w_k / 1000.0),
            Quantity::length_m(limit / 1000.0),
            "liquid retaining crack width SLS",
            AnnexChoice::En,
        )
    }

    pub fn check_steel_stress(sigma_s_mpa: f64, exposure: &str) -> CheckResult {
        let limit = steel_stress_limit_mpa(exposure);
        CheckResult::from_utilization(
            ClauseId::new("EN 1992-3", "§7", "7.2"),
            Quantity::stress_mpa(sigma_s_mpa),
            Quantity::stress_mpa(limit),
            "liquid retaining steel stress SLS",
            AnnexChoice::En,
        )
    }
}
// #endregion 🔖Part3

// #region 🔖Part4
pub mod part_4 {
    use super::{part_1_1, AnnexChoice, CheckResult};

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

// #region 🔖Fem
use fem_core::{BeamEb2, Dof, MemberUdl, Model, Node, Support};

fn max_beam_moment_knm(result: &fem_core::StaticResult, element_id: &str) -> f64 {
    let (_, fem_core::ElementResult::Beam { stations }) = result
        .elements
        .iter()
        .find(|(id, _)| id == element_id)
        .expect("beam element result")
    else {
        panic!("expected beam element result");
    };
    stations
        .iter()
        .map(|s| s.m.abs())
        .fold(0.0_f64, f64::max)
        / 1000.0
}

fn max_beam_shear_kn(result: &fem_core::StaticResult, element_id: &str) -> f64 {
    let (_, fem_core::ElementResult::Beam { stations }) = result
        .elements
        .iter()
        .find(|(id, _)| id == element_id)
        .expect("beam element result")
    else {
        panic!("expected beam element result");
    };
    stations
        .iter()
        .map(|s| s.v.abs())
        .fold(0.0_f64, f64::max)
        / 1000.0
}

/// 🏗️ Solve a simply supported RC beam with `fem_core` and run EN 1992 ULS checks.
pub fn check_rc_beam_from_fem(
    span_m: f64,
    udl_kn_m: f64,
    f_ck: f64,
    b_mm: f64,
    d_mm: f64,
    a_s_mm2: f64,
    f_yk: f64,
    rho_l: f64,
) -> Result<CheckReport, fem_core::FemError> {
    let mut model = Model::default();
    model.nodes.push(Node {
        id: "n0".into(),
        pos: [0.0, 0.0, 0.0],
    });
    model.nodes.push(Node {
        id: "n1".into(),
        pos: [span_m, 0.0, 0.0],
    });
    model.supports.push(Support {
        node_id: "n0".into(),
        fixed: vec![Dof::Tx, Dof::Ty],
    });
    model.supports.push(Support {
        node_id: "n1".into(),
        fixed: vec![Dof::Ty],
    });
    model.elements.push(Box::new(BeamEb2 {
        id: "b1".into(),
        start: "n0".into(),
        end: "n1".into(),
        e: 30e9,
        area: b_mm * d_mm / 1e6,
        iy: b_mm * d_mm.powi(3) / 12e12,
        density: 2500.0,
    }));
    model.member_loads.push((
        "b1".into(),
        MemberUdl {
            wx: 0.0,
            wy: -udl_kn_m * 1000.0,
            wz: 0.0,
        },
    ));

    let result = fem_core::solve_linear_static(&model)?;
    let m_ed_knm = max_beam_moment_knm(&result, "b1");
    let v_ed_kn = max_beam_shear_kn(&result, "b1");

    Ok(check_rc_beam(
        m_ed_knm,
        v_ed_kn,
        f_ck,
        b_mm,
        d_mm,
        a_s_mm2,
        f_yk,
        rho_l,
        0.0,
    ))
}
// #endregion 🔖Fem

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rc_beam_e2e() {
        let report = check_rc_beam(120.0, 80.0, 30.0, 300.0, 500.0, 2500.0, 500.0, 0.01, 200.0);
        assert!(!report.checks.is_empty());
        assert!(report.checks[0].utilization > 0.0);
    }

    #[test]
    fn punching_v_rd_max_c30() {
        let v = part_1_1::punching_v_rd_max_mpa(30.0);
        assert!((v - 4.488).abs() < 0.1);
    }

    #[test]
    fn slenderness_column() {
        let i = part_1_1::radius_of_gyration_mm(300_000.0, 2.25e9);
        let lambda = part_1_1::slenderness_lambda(3000.0, i);
        assert!((lambda - 34.6).abs() < 1.0);
    }

    #[test]
    fn crack_width_wk() {
        let eps_sm = part_1_1::steel_strain_eps_sm(200.0, 0.01, 2.9, 200_000.0);
        let wk = part_1_1::crack_width_wk_mm(eps_sm, 0.0001, 300.0);
        assert!(wk > 0.0 && wk < 0.5);
    }

    #[test]
    fn deflection_ss_udl() {
        let delta = part_1_1::deflection_ss_udl_mm(20.0, 6.0, 30_000.0, 1.875e9);
        assert!((delta - 6.0).abs() < 0.5);
    }

    #[test]
    fn fire_cover_beam_r60() {
        let req = part_1_2::min_axis_distance_mm(part_1_2::ElementType::Beam, part_1_2::FireRating::R60);
        assert!((req - 35.0).abs() < 0.1);
    }

    #[test]
    fn liquid_retaining_stress_limit() {
        assert!((part_3::steel_stress_limit_mpa("XD1") - 160.0).abs() < 0.1);
        let wk = part_3::crack_width_liquid_mm(220.0, "XD1", 250.0, 200_000.0);
        assert!(wk < 0.25);
    }

    #[test]
    fn na_de_alpha_cc() {
        assert!((na_de::alpha_cc() - 0.85).abs() < 1e-9);
    }

    #[test]
    fn rc_beam_from_fem_e2e() {
        let report = check_rc_beam_from_fem(6.0, 20.0, 30.0, 300.0, 500.0, 2500.0, 500.0, 0.01)
            .expect("fem solve");
        assert!(!report.checks.is_empty());
        let m_ed = report.checks[0].computed.value / 1_000_000.0;
        assert!((m_ed - 90.0).abs() < 1.0);
    }
}
