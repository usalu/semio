//! ⚙️ EN 1999 app — headless compute (constitutional: engine).

use crate::artifacts::en1999::Document;
use crate::core::{AnnexChoice, CheckReport, CheckResult, ClauseId, Quantity};

// #region 🔖️NaDe
pub mod na_de {
    use crate::core::AnnexChoice;
    pub use crate::artifacts::en1990::engine::na_de::NaDe;

    pub const HAZ_ZONE_MM: f64 = 25.0;

    /// 🇪️🇺️ EN 1999-1-1 §6.1.3 material partial factors, resolved per national annex choice.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct AnnexParams {
        pub choice: AnnexChoice,
        pub gamma_m1: f64,
        pub gamma_m2: f64,
    }

    impl AnnexParams {
        pub fn en() -> Self {
            Self { choice: AnnexChoice::En, gamma_m1: 1.1, gamma_m2: 1.25 }
        }

        /// 🇩️🇪️ DIN EN 1999-1-1/NA sets no NDP override for γ_M1/γ_M2 — equal to EN by design, not an oversight.
        pub fn de() -> Self {
            Self { choice: AnnexChoice::De, gamma_m1: 1.1, gamma_m2: 1.25 }
        }

        pub fn for_choice(choice: AnnexChoice) -> Self {
            match choice {
                AnnexChoice::En => Self::en(),
                AnnexChoice::De => Self::de(),
            }
        }
    }
}
// #endregion 🔖️NaDe

// #region 🔖️Part1_1
pub mod part_1_1 {
    use super::*;

    /// 🔩️ Aluminium alloy tempers per EN 1999-1-1 Table 3.1.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Alloy {
        Aw6060T6,
        Aw6082T6,
    }

    impl Alloy {
        pub fn f_0_2_mpa(self) -> f64 {
            match self {
                Self::Aw6060T6 => 190.0,
                Self::Aw6082T6 => 260.0,
            }
        }

        pub fn f_u_mpa(self) -> f64 {
            match self {
                Self::Aw6060T6 => 215.0,
                Self::Aw6082T6 => 310.0,
            }
        }

        pub fn epsilon(self) -> f64 {
            (250.0 / self.f_0_2_mpa()).sqrt()
        }

        /// 🔥️ HAZ softened strength f_o,haz [MPa], representative GMAW-welded value per EN 1999-1-1 Table 6.2.
        pub fn f_o_haz_mpa(self) -> f64 {
            match self {
                Self::Aw6060T6 => 95.0,
                Self::Aw6082T6 => 185.0,
            }
        }
    }

    /// 📐️ Section class per EN 1999-1-1 Table 6.2.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SectionClass {
        Class1,
        Class2,
        Class3,
        Class4,
    }

    pub fn classify_flange_outstand(b_mm: f64, t_mm: f64, alloy: Alloy) -> SectionClass {
        let eps = alloy.epsilon();
        let ratio = b_mm / t_mm;
        if ratio <= 3.0 * eps {
            SectionClass::Class1
        } else if ratio <= 5.0 * eps {
            SectionClass::Class2
        } else if ratio <= 8.0 * eps {
            SectionClass::Class3
        } else {
            SectionClass::Class4
        }
    }

    /// 📐️ Bending resistance M_c,Rd [kNm] per EN 1999-1-1 §6.2.5.
    pub fn m_c_rd_knm(w_el_mm3: f64, alloy: Alloy, gamma_m: f64) -> f64 {
        w_el_mm3 * alloy.f_0_2_mpa() / gamma_m / 1_000_000.0
    }

    pub fn check_cross_section(n_ed: f64, n_rd: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1999-1-1", "§6.2", "6.2"), Quantity::force_kn(n_ed), Quantity::force_kn(n_rd), "aluminium cross-section ULS", annex)
    }

    pub fn check_buckling(n_ed: f64, n_b_rd: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1999-1-1", "§6.3", "6.3"), Quantity::force_kn(n_ed), Quantity::force_kn(n_b_rd), "aluminium buckling ULS", annex)
    }

    pub fn check_bending(m_ed_knm: f64, m_rd_knm: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1999-1-1", "§6.2.5", "6.2.5"),
            Quantity::new(crate::core::QuantityKind::Moment, m_ed_knm * 1_000_000.0),
            Quantity::new(crate::core::QuantityKind::Moment, m_rd_knm * 1_000_000.0),
            "aluminium bending ULS",
            annex,
        )
    }

    /// 🔥️ HAZ reduced strength f_o,haz [MPa] in the HAZ zone near welds.
    pub fn haz_strength_mpa(alloy: Alloy) -> f64 {
        alloy.f_o_haz_mpa()
    }

    /// 🔥️ HAZ strength reduction factor ρ_o,haz = f_o,haz / f_o per EN 1999-1-1 Table 6.2, applied within the HAZ zone near welds.
    pub fn haz_reduction_factor(alloy: Alloy) -> f64 {
        alloy.f_o_haz_mpa() / alloy.f_0_2_mpa()
    }

    /// 📐️ Torsional buckling resistance N_b,Rd [kN] with torsion constant I_t.
    pub fn torsional_buckling_resistance_kn(a_mm2: f64, i_t_mm4: f64, l_cr_mm: f64, alloy: Alloy, gamma_m: f64, e_mpa: f64) -> f64 {
        let g = e_mpa / (2.0 * (1.0 + 0.33));
        let c_t = (g * i_t_mm4 / (l_cr_mm * l_cr_mm)).sqrt();
        let lambda = (a_mm2 * alloy.f_0_2_mpa() / (c_t * gamma_m)).sqrt();
        let chi = if lambda <= 0.2 { 1.0 } else { (1.0 / (0.5 * (1.0 + 0.21 * (lambda - 0.2) + lambda * lambda))).min(1.0) };
        chi * a_mm2 * alloy.f_0_2_mpa() / gamma_m / 1000.0
    }

    /// 🔗️ Directional weld throat resistance F_w,Rd [kN] per EN 1999-1-1 §8.5 (absorbed here — welds are not a dedicated EN 1999 part).
    pub fn weld_resistance_kn(a_w_mm2: f64, f_u_mpa: f64, beta_w: f64, gamma_m2: f64) -> f64 {
        a_w_mm2 * f_u_mpa / (beta_w * gamma_m2 * 1000.0)
    }

    /// 🔗️ Weld throat area A_w = a · l [mm²].
    pub fn weld_throat_area_mm2(throat_mm: f64, length_mm: f64) -> f64 {
        throat_mm * length_mm
    }

    pub fn check_welded_joint(v_ed_kn: f64, v_rd_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1999-1-1", "§8.5", "8.5"), Quantity::force_kn(v_ed_kn), Quantity::force_kn(v_rd_kn), "aluminium welded joint ULS", annex)
    }
}
// #endregion 🔖️Part1_1

// #region 🔖️Part1_2
pub mod part_1_2 {
    use super::*;

    /// 🔥️ Critical temperature θ_cr [°C] for fire protection assessment.
    pub fn critical_temperature_c(f_0_2_mpa: f64) -> f64 {
        170.0 + 0.4 * f_0_2_mpa
    }

    /// 🔥️ Strength reduction factor k_θ at elevated temperature.
    pub fn strength_reduction_factor(theta_c: f64, theta_cr_c: f64) -> f64 {
        if theta_c <= theta_cr_c {
            1.0
        } else {
            ((theta_cr_c + 200.0 - theta_c) / 200.0).clamp(0.0, 1.0)
        }
    }

    pub fn check_fire_protection(theta_c: f64, theta_limit_c: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1999-1-2", "§4", "4.2"), Quantity::new(crate::core::QuantityKind::Temperature, theta_c), Quantity::new(crate::core::QuantityKind::Temperature, theta_limit_c), "aluminium fire protection", annex)
    }
}
// #endregion 🔖️Part1_2

// #region 🔖️Part1_3
pub mod part_1_3 {
    use super::*;

    pub const FATIGUE_CUTOFF_CYCLES: f64 = 2_000_000.0;

    /// 🔄️ Fatigue strength at N cycles [MPa] per EN 1999-1-3.
    pub fn fatigue_strength_mpa(delta_sigma_c_mpa: f64, m: f64, n_cycles: f64) -> f64 {
        if n_cycles >= FATIGUE_CUTOFF_CYCLES {
            0.0
        } else {
            delta_sigma_c_mpa * (2_000_000.0 / n_cycles).powf(1.0 / m)
        }
    }

    pub fn check_fatigue(delta_sigma_ed: f64, delta_sigma_rd: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1999-1-3", "§7", "7.1"), Quantity::stress_mpa(delta_sigma_ed), Quantity::stress_mpa(delta_sigma_rd), "aluminium fatigue ULS", annex)
    }
}
// #endregion 🔖️Part1_3

// #region 🔖️Part1_4
pub mod part_1_4 {
    use super::part_1_1::Alloy;
    use super::*;

    /// 📄️ Plate slenderness λ_p for a cold-formed compression element per EN 1999-1-4 §5 (effective width method).
    pub fn plate_slenderness(b_mm: f64, t_mm: f64, k_sigma: f64, alloy: Alloy) -> f64 {
        let eps = alloy.epsilon();
        (b_mm / t_mm) / (28.4 * eps * k_sigma.sqrt())
    }

    /// 📄️ Effective width reduction factor ρ per EN 1999-1-4 §5.4.
    pub fn effective_width_factor(lambda_p: f64) -> f64 {
        if lambda_p <= 0.673 {
            1.0
        } else {
            ((1.0 - 0.22 / lambda_p) / lambda_p).min(1.0)
        }
    }

    /// 📄️ Effective elastic section modulus W_eff [mm³] of a class 4 cold-formed cross-section.
    pub fn effective_section_modulus_mm3(w_el_mm3: f64, rho: f64) -> f64 {
        w_el_mm3 * rho
    }

    pub fn check_cold_formed_sheeting(m_ed_knm: f64, w_eff_mm3: f64, alloy: Alloy, gamma_m1: f64, annex: AnnexChoice) -> CheckResult {
        let m_rd_knm = w_eff_mm3 * alloy.f_0_2_mpa() / gamma_m1 / 1_000_000.0;
        CheckResult::from_utilization(
            ClauseId::new("EN 1999-1-4", "§5.4", "5.4"),
            Quantity::new(crate::core::QuantityKind::Moment, m_ed_knm * 1_000_000.0),
            Quantity::new(crate::core::QuantityKind::Moment, m_rd_knm * 1_000_000.0),
            "cold-formed sheeting effective bending resistance",
            annex,
        )
    }
}
// #endregion 🔖️Part1_4

// #region 🔖️Part1_5
pub mod part_1_5 {
    use super::*;

    /// 🛢️ Aluminium elastic modulus [MPa] used for shell buckling per EN 1999-1-5.
    pub const E_ALUMINIUM_MPA: f64 = 70_000.0;

    /// 🛢️ Cylindrical shell axial elastic critical buckling stress σ_cr [MPa] per EN 1999-1-5 §5.
    pub fn critical_axial_stress_mpa(t_mm: f64, r_mm: f64) -> f64 {
        0.605 * E_ALUMINIUM_MPA * t_mm / r_mm
    }

    /// 🛢️ Relative shell slenderness λ̄ = √(f_o / σ_cr).
    pub fn relative_slenderness(f_o_mpa: f64, sigma_cr_mpa: f64) -> f64 {
        (f_o_mpa / sigma_cr_mpa).sqrt()
    }

    /// 🛢️ Shell buckling reduction factor χ from relative slenderness λ̄.
    pub fn buckling_reduction_factor(lambda_bar: f64) -> f64 {
        if lambda_bar <= 0.2 {
            1.0
        } else {
            (1.0 / (0.5 * (1.0 + 0.21 * (lambda_bar - 0.2) + lambda_bar * lambda_bar))).min(1.0)
        }
    }

    /// 🛢️ Design shell buckling resistance stress σ_Rd = χ·f_o/γ_M1 [MPa].
    pub fn design_buckling_stress_mpa(chi: f64, f_o_mpa: f64, gamma_m1: f64) -> f64 {
        chi * f_o_mpa / gamma_m1
    }

    pub fn check_shell_buckling(sigma_ed_mpa: f64, sigma_rd_mpa: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1999-1-5", "§5", "5.3"), Quantity::stress_mpa(sigma_ed_mpa), Quantity::stress_mpa(sigma_rd_mpa), "cylindrical shell axial buckling", annex)
    }
}
// #endregion 🔖️Part1_5

/// 📐️ Aluminium bending resistance M_c,Rd [kNm].
pub fn bending_resistance_knm(w_el_mm3: f64, f_0_2_mpa: f64, gamma_m: f64) -> f64 {
    w_el_mm3 * f_0_2_mpa / gamma_m / 1_000_000.0
}

/// 📐️ Buckling resistance with χ factor.
pub fn buckling_resistance_kn(a_mm2: f64, f_0_2_mpa: f64, chi: f64, gamma_m: f64) -> f64 {
    chi * a_mm2 * f_0_2_mpa / gamma_m / 1000.0
}

/// 📋️ Aluminium member check.
pub fn check_aluminium_member(n_ed_kn: f64, m_ed_knm: f64, a_mm2: f64, w_el_mm3: f64, alloy: part_1_1::Alloy, chi: f64, i_t_mm4: f64, l_cr_mm: f64, annex: AnnexChoice) -> CheckReport {
    let gamma_m1 = na_de::AnnexParams::for_choice(annex).gamma_m1;
    let f_0_2 = alloy.f_0_2_mpa();
    let n_rd = a_mm2 * f_0_2 / gamma_m1 / 1000.0;
    let n_b_rd = buckling_resistance_kn(a_mm2, f_0_2, chi, gamma_m1);
    let n_t_rd = part_1_1::torsional_buckling_resistance_kn(a_mm2, i_t_mm4, l_cr_mm, alloy, gamma_m1, 70_000.0);
    let m_rd = part_1_1::m_c_rd_knm(w_el_mm3, alloy, gamma_m1);
    let mut report = CheckReport::default();
    report.push(part_1_1::check_cross_section(n_ed_kn, n_rd, annex));
    report.push(part_1_1::check_buckling(n_ed_kn, n_b_rd.min(n_t_rd), annex));
    report.push(part_1_1::check_bending(m_ed_knm, m_rd, annex));
    report
}

/// 📋️ Full EN 1999 check spanning every remaining part: 1-1 (cross-section, buckling, bending, welds), 1-2 (fire), 1-3 (fatigue), 1-4 (cold-formed sheeting), 1-5 (shell buckling).
#[allow(clippy::too_many_arguments)]
pub fn check_full_aluminium(
    n_ed_kn: f64,
    m_ed_knm: f64,
    a_mm2: f64,
    w_el_mm3: f64,
    alloy: part_1_1::Alloy,
    chi: f64,
    i_t_mm4: f64,
    l_cr_mm: f64,
    theta_c: f64,
    delta_sigma_ed: f64,
    delta_sigma_c: f64,
    fatigue_m: f64,
    n_cycles: f64,
    v_weld_ed_kn: f64,
    weld_throat_mm: f64,
    weld_length_mm: f64,
    beta_w: f64,
    sheet_b_mm: f64,
    sheet_t_mm: f64,
    sheet_k_sigma: f64,
    sheet_w_el_mm3: f64,
    sheet_m_ed_knm: f64,
    shell_t_mm: f64,
    shell_r_mm: f64,
    sigma_ed_shell_mpa: f64,
    annex: AnnexChoice,
) -> CheckReport {
    let params = na_de::AnnexParams::for_choice(annex);
    let mut report = check_aluminium_member(n_ed_kn, m_ed_knm, a_mm2, w_el_mm3, alloy, chi, i_t_mm4, l_cr_mm, annex);
    let theta_cr = part_1_2::critical_temperature_c(alloy.f_0_2_mpa());
    report.push(part_1_2::check_fire_protection(theta_c, theta_cr, annex));
    let delta_sigma_rd = part_1_3::fatigue_strength_mpa(delta_sigma_c, fatigue_m, n_cycles);
    report.push(part_1_3::check_fatigue(delta_sigma_ed, delta_sigma_rd, annex));
    let a_w = part_1_1::weld_throat_area_mm2(weld_throat_mm, weld_length_mm);
    let v_weld_rd = part_1_1::weld_resistance_kn(a_w, alloy.f_u_mpa(), beta_w, params.gamma_m2);
    report.push(part_1_1::check_welded_joint(v_weld_ed_kn, v_weld_rd, annex));
    let lambda_p = part_1_4::plate_slenderness(sheet_b_mm, sheet_t_mm, sheet_k_sigma, alloy);
    let rho = part_1_4::effective_width_factor(lambda_p);
    let w_eff = part_1_4::effective_section_modulus_mm3(sheet_w_el_mm3, rho);
    report.push(part_1_4::check_cold_formed_sheeting(sheet_m_ed_knm, w_eff, alloy, params.gamma_m1, annex));
    let sigma_cr = part_1_5::critical_axial_stress_mpa(shell_t_mm, shell_r_mm);
    let lambda_bar = part_1_5::relative_slenderness(alloy.f_0_2_mpa(), sigma_cr);
    let chi_shell = part_1_5::buckling_reduction_factor(lambda_bar);
    let sigma_rd_shell = part_1_5::design_buckling_stress_mpa(chi_shell, alloy.f_0_2_mpa(), params.gamma_m1);
    report.push(part_1_5::check_shell_buckling(sigma_ed_shell_mpa, sigma_rd_shell, annex));
    report
}

// #region 🔖️Session
fn parse_alloy(value: &str) -> part_1_1::Alloy {
    match value.to_ascii_lowercase().as_str() {
        "aw6082t6" => part_1_1::Alloy::Aw6082T6,
        _ => part_1_1::Alloy::Aw6060T6,
    }
}

/// 🧮️ Headless per-document evaluation — the `NormFamily::evaluate` body for `En1999Family` (defined
/// in the sibling `op` crate, which depends on this `engine` crate to call it).
pub fn evaluate(document: &Document) -> CheckReport {
    check_full_aluminium(
        document.n_ed_kn,
        document.m_ed_knm,
        document.a_mm2,
        document.w_el_mm3,
        parse_alloy(&document.alloy),
        document.chi,
        document.i_t_mm4,
        document.l_cr_mm,
        document.theta_c,
        document.delta_sigma_ed,
        document.delta_sigma_c,
        document.fatigue_m,
        document.n_cycles,
        document.v_weld_ed_kn,
        document.weld_throat_mm,
        document.weld_length_mm,
        document.beta_w,
        document.sheet_b_mm,
        document.sheet_t_mm,
        document.sheet_k_sigma,
        document.sheet_w_el_mm3,
        document.sheet_m_ed_knm,
        document.shell_t_mm,
        document.shell_r_mm,
        document.sigma_ed_shell_mpa,
        document.annex,
    )
}
// #endregion 🔖️Session

// #region 🔖️Session
/// 🧩️ EN 1999's `NormFamily` binding — ties this artifact's `Document` to the `evaluate` above for the
/// headless `NormHost` session every norm app drives.
pub struct En1999Family;

impl crate::core::NormFamily for En1999Family {
    type Document = Document;
    type Operation = crate::artifacts::en1999::op::Operation;

    fn family_id() -> crate::core::NormFamilyId {
        crate::core::NormFamilyId::En1999
    }

    fn evaluate(document: &Document) -> CheckReport {
        evaluate(document)
    }
}

pub type Host = crate::core::NormHost<En1999Family>;
// #endregion 🔖️Session

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alloy_6060_t6_m_c_rd() {
        let alloy = part_1_1::Alloy::Aw6060T6;
        let w_el = 24_000.0;
        let m_rd = part_1_1::m_c_rd_knm(w_el, alloy, na_de::AnnexParams::de().gamma_m1);
        assert!((m_rd - 4.145454545).abs() < 1e-6);
    }

    #[test]
    fn section_classification_6060() {
        let alloy = part_1_1::Alloy::Aw6060T6;
        let class = part_1_1::classify_flange_outstand(10.0, 3.0, alloy);
        assert_eq!(class, part_1_1::SectionClass::Class1);
    }

    #[test]
    fn haz_reduced_strength() {
        let alloy = part_1_1::Alloy::Aw6060T6;
        let f_haz = part_1_1::haz_strength_mpa(alloy);
        assert!((f_haz - 95.0).abs() < 1e-9);
        assert!((na_de::HAZ_ZONE_MM - 25.0).abs() < 1e-9);
    }

    #[test]
    fn haz_reduction_factor_6082_t6_table_6_2() {
        // 📖️ EN 1999-1-1 Table 6.2 representative GMAW HAZ value for 6082-T6: f_o,haz ≈ 185 MPa vs f_o = 260 MPa ⇒ ρ_o,haz ≈ 0.71.
        let alloy = part_1_1::Alloy::Aw6082T6;
        let rho_o_haz = part_1_1::haz_reduction_factor(alloy);
        assert!((rho_o_haz - 185.0 / 260.0).abs() < 1e-9);
        assert!((rho_o_haz - 0.711).abs() < 1e-3);
    }

    #[test]
    fn fatigue_cutoff_at_2e6() {
        let strength = part_1_3::fatigue_strength_mpa(80.0, 8.0, 2_000_000.0);
        assert!((strength - 0.0).abs() < 1e-9);
        let below = part_1_3::fatigue_strength_mpa(80.0, 8.0, 1_000_000.0);
        assert!(below > 0.0);
    }

    #[test]
    fn aluminium_member_e2e() {
        let alloy = part_1_1::Alloy::Aw6082T6;
        let report = check_aluminium_member(80.0, 12.0, 1200.0, 15_000.0, alloy, 0.8, 5000.0, 3000.0, AnnexChoice::De);
        assert_eq!(report.checks.len(), 3);
    }

    #[test]
    fn weld_resistance_worked() {
        let a_w = part_1_1::weld_throat_area_mm2(4.0, 120.0);
        assert!((a_w - 480.0).abs() < 1e-9);
        let gamma_m2 = na_de::AnnexParams::de().gamma_m2;
        let v_rd = part_1_1::weld_resistance_kn(a_w, 215.0, 0.63, gamma_m2);
        assert!((v_rd - 480.0 * 215.0 / (0.63 * gamma_m2 * 1000.0)).abs() < 1e-6);
    }

    #[test]
    fn fire_critical_temperature_6060() {
        let theta_cr = part_1_2::critical_temperature_c(190.0);
        assert!((theta_cr - 246.0).abs() < 0.1);
        let k_theta = part_1_2::strength_reduction_factor(300.0, theta_cr);
        assert!(k_theta < 1.0);
    }

    #[test]
    fn cold_formed_effective_width_factor_worked() {
        // 📖️ EN 1999-1-4 §5.4: ρ(λ_p=1.0) = (1 − 0.22/1.0)/1.0 = 0.78.
        let rho = part_1_4::effective_width_factor(1.0);
        assert!((rho - 0.78).abs() < 1e-9);
    }

    #[test]
    fn cold_formed_effective_width_factor_below_limit_is_unity() {
        let rho = part_1_4::effective_width_factor(0.5);
        assert!((rho - 1.0).abs() < 1e-9);
    }

    #[test]
    fn check_cold_formed_sheeting_worked() {
        let alloy = part_1_1::Alloy::Aw6060T6;
        let gamma_m1 = na_de::AnnexParams::de().gamma_m1;
        let w_eff = part_1_4::effective_section_modulus_mm3(8000.0, 0.78);
        let expected_m_rd_knm = w_eff * alloy.f_0_2_mpa() / gamma_m1 / 1_000_000.0;
        let result = part_1_4::check_cold_formed_sheeting(0.5, w_eff, alloy, gamma_m1, AnnexChoice::De);
        assert!((result.limit.value / 1_000_000.0 - expected_m_rd_knm).abs() < 1e-9);
    }

    #[test]
    fn shell_critical_axial_stress_worked() {
        // 📖️ EN 1999-1-5 §5: σ_cr = 0.605·E·t/r = 0.605·70000·4/500 = 338.8 MPa.
        let sigma_cr = part_1_5::critical_axial_stress_mpa(4.0, 500.0);
        assert!((sigma_cr - 338.8).abs() < 1e-6);
    }

    #[test]
    fn shell_buckling_pipeline_worked() {
        let alloy = part_1_1::Alloy::Aw6060T6;
        let gamma_m1 = na_de::AnnexParams::de().gamma_m1;
        let sigma_cr = part_1_5::critical_axial_stress_mpa(4.0, 500.0);
        let lambda_bar = part_1_5::relative_slenderness(alloy.f_0_2_mpa(), sigma_cr);
        assert!((lambda_bar - (alloy.f_0_2_mpa() / sigma_cr).sqrt()).abs() < 1e-9);
        let chi = part_1_5::buckling_reduction_factor(lambda_bar);
        let sigma_rd = part_1_5::design_buckling_stress_mpa(chi, alloy.f_0_2_mpa(), gamma_m1);
        assert!((sigma_rd - chi * alloy.f_0_2_mpa() / gamma_m1).abs() < 1e-9);
        let result = part_1_5::check_shell_buckling(150.0, sigma_rd, AnnexChoice::De);
        assert!(result.utilization < 1.0);
    }

    #[test]
    fn full_aluminium_worked_example() {
        let alloy = part_1_1::Alloy::Aw6060T6;
        let report = check_full_aluminium(80.0, 4.0, 1200.0, 24_000.0, alloy, 0.85, 5000.0, 3000.0, 200.0, 45.0, 71.0, 8.0, 500_000.0, 25.0, 4.0, 120.0, 0.63, 200.0, 2.0, 4.0, 8000.0, 0.5, 4.0, 500.0, 150.0, AnnexChoice::De);
        assert_eq!(report.checks.len(), 8);
        assert!(report.checks[4].utilization < 1.0);
    }

    #[test]
    fn evaluate_runs_all_parts() {
        let report = evaluate(&Document::default());
        assert_eq!(report.checks.len(), 8);
    }

    #[test]
    fn annex_en_de_documented_equality() {
        // 📖️ DIN EN 1999-1-1/NA does not override γ_M1/γ_M2, so EN and DE-NA must yield identical utilization.
        let en_doc = Document { annex: AnnexChoice::En, ..Document::default() };
        let de_doc = Document { annex: AnnexChoice::De, ..Document::default() };
        let en_report = evaluate(&en_doc);
        let de_report = evaluate(&de_doc);
        assert_eq!(en_report.checks.len(), de_report.checks.len());
        for (en_check, de_check) in en_report.checks.iter().zip(de_report.checks.iter()) {
            assert!((en_check.utilization - de_check.utilization).abs() < 1e-9);
        }
    }
}
//#endregion 🧪️Tests
