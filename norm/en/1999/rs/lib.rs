//! ✨ EN 1999 design of aluminium structures.

use norm_core::{AnnexChoice, CheckReport, CheckResult, ClauseId, Quantity};
use serde::{Deserialize, Serialize};

// #region 🔖NaDe
pub mod na_de {
    pub use norm_en_1990::na_de::NaDe;

    pub const GAMMA_M: f64 = 1.1;
    pub const HAZ_ZONE_MM: f64 = 25.0;
    pub const HAZ_STRENGTH_FACTOR: f64 = 0.5;
}
// #endregion 🔖NaDe

// #region 🔖Part1_1
pub mod part_1_1 {
    use super::*;

    /// 🔩 Aluminium alloy tempers per EN 1999-1-1 Table 3.1.
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
    }

    /// 📐 Section class per EN 1999-1-1 Table 6.2.
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

    /// 📐 Bending resistance M_c,Rd [kNm] per EN 1999-1-1 §6.2.5.
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
            Quantity::new(norm_core::QuantityKind::Moment, m_ed_knm * 1_000_000.0),
            Quantity::new(norm_core::QuantityKind::Moment, m_rd_knm * 1_000_000.0),
            "aluminium bending ULS",
            annex,
        )
    }

    /// 🔥 HAZ reduced strength f_0.2,HAZ [MPa] in 25 mm zone.
    pub fn haz_strength_mpa(alloy: Alloy) -> f64 {
        na_de::HAZ_STRENGTH_FACTOR * alloy.f_0_2_mpa()
    }

    /// 📐 Torsional buckling resistance N_b,Rd [kN] with torsion constant I_t.
    pub fn torsional_buckling_resistance_kn(a_mm2: f64, i_t_mm4: f64, l_cr_mm: f64, alloy: Alloy, gamma_m: f64, e_mpa: f64) -> f64 {
        let g = e_mpa / (2.0 * (1.0 + 0.33));
        let c_t = (g * i_t_mm4 / (l_cr_mm * l_cr_mm)).sqrt();
        let lambda = (a_mm2 * alloy.f_0_2_mpa() / (c_t * gamma_m)).sqrt();
        let chi = if lambda <= 0.2 { 1.0 } else { (1.0 / (0.5 * (1.0 + 0.21 * (lambda - 0.2) + lambda * lambda))).min(1.0) };
        chi * a_mm2 * alloy.f_0_2_mpa() / gamma_m / 1000.0
    }
}
// #endregion 🔖Part1_1

// #region 🔖Part1_2
pub mod part_1_2 {
    use super::*;

    /// 🔥 Critical temperature θ_cr [°C] for fire protection assessment.
    pub fn critical_temperature_c(f_0_2_mpa: f64) -> f64 {
        170.0 + 0.4 * f_0_2_mpa
    }

    /// 🔥 Strength reduction factor k_θ at elevated temperature.
    pub fn strength_reduction_factor(theta_c: f64, theta_cr_c: f64) -> f64 {
        if theta_c <= theta_cr_c {
            1.0
        } else {
            ((theta_cr_c + 200.0 - theta_c) / 200.0).clamp(0.0, 1.0)
        }
    }

    pub fn check_fire_protection(theta_c: f64, theta_limit_c: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1999-1-2", "§4", "4.2"),
            Quantity::new(norm_core::QuantityKind::Temperature, theta_c),
            Quantity::new(norm_core::QuantityKind::Temperature, theta_limit_c),
            "aluminium fire protection",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖Part1_2

// #region 🔖Part1_3
pub mod part_1_3 {
    use super::*;

    pub const FATIGUE_CUTOFF_CYCLES: f64 = 2_000_000.0;

    /// 🔄 Fatigue strength at N cycles [MPa] per EN 1999-1-3.
    pub fn fatigue_strength_mpa(delta_sigma_c_mpa: f64, m: f64, n_cycles: f64) -> f64 {
        if n_cycles >= FATIGUE_CUTOFF_CYCLES {
            0.0
        } else {
            delta_sigma_c_mpa * (2_000_000.0 / n_cycles).powf(1.0 / m)
        }
    }

    pub fn check_fatigue(delta_sigma_ed: f64, delta_sigma_rd: f64) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1999-1-3", "§7", "7.1"), Quantity::stress_mpa(delta_sigma_ed), Quantity::stress_mpa(delta_sigma_rd), "aluminium fatigue ULS", AnnexChoice::En)
    }
}
// #endregion 🔖Part1_3

// #region 🔖Part1_4
pub mod part_1_4 {
    use super::*;

    /// 🍽️ Food-contact surface coating thickness minimum [µm].
    pub fn min_coating_thickness_um() -> f64 {
        25.0
    }

    /// 🍽️ Maximum allowable lead content for food contact [ppm].
    pub fn max_lead_content_ppm() -> f64 {
        100.0
    }

    pub fn check_food_contact_surface(coating_um: f64) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1999-1-4", "§5", "5.2"), Quantity::length_m(coating_um / 1_000_000.0), Quantity::length_m(min_coating_thickness_um() / 1_000_000.0), "food contact coating thickness", AnnexChoice::De)
    }

    pub fn check_lead_content(lead_ppm: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1999-1-4", "§5", "5.3"),
            Quantity::new(norm_core::QuantityKind::Dimensionless, lead_ppm),
            Quantity::new(norm_core::QuantityKind::Dimensionless, max_lead_content_ppm()),
            "food contact lead content",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖Part1_4

// #region 🔖Part1_5
pub mod part_1_5 {
    use super::part_1_1::{Alloy, SectionClass};
    use super::*;

    /// 🔲 Hollow section effective area A_eff [mm²] for class 4.
    pub fn effective_area_mm2(a_gross_mm2: f64, b_mm: f64, t_mm: f64, alloy: Alloy) -> f64 {
        let class = part_1_1::classify_flange_outstand(b_mm, t_mm, alloy);
        match class {
            SectionClass::Class4 => a_gross_mm2 * 0.9,
            _ => a_gross_mm2,
        }
    }

    /// 🔲 Hollow section torsion constant I_t [mm⁴] for rectangular RHS.
    pub fn rectangular_rhs_i_t_mm4(h_mm: f64, b_mm: f64, t_mm: f64) -> f64 {
        let h_i = h_mm - 2.0 * t_mm;
        let b_i = b_mm - 2.0 * t_mm;
        2.0 * t_mm * h_i * h_i * b_i * b_i / (h_i + b_i)
    }

    pub fn check_hollow_section(n_ed: f64, n_rd: f64) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1999-1-5", "§6", "6.2"), Quantity::force_kn(n_ed), Quantity::force_kn(n_rd), "extruded hollow section ULS", AnnexChoice::De)
    }
}
// #endregion 🔖Part1_5

// #region 🔖Part1_6
pub mod part_1_6 {
    use super::*;

    /// 🔗 Directional weld throat resistance F_w,Rd [kN] per EN 1999-1-1 §8.5.
    pub fn weld_resistance_kn(a_w_mm2: f64, f_u_mpa: f64, beta_w: f64, gamma_m: f64) -> f64 {
        a_w_mm2 * f_u_mpa / (beta_w * gamma_m * 1000.0)
    }

    /// 🔗 Weld throat area A_w = a · l [mm²].
    pub fn weld_throat_area_mm2(throat_mm: f64, length_mm: f64) -> f64 {
        throat_mm * length_mm
    }

    pub fn check_welded_joint(v_ed_kn: f64, v_rd_kn: f64) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1999-1-1", "§8.5", "8.5"), Quantity::force_kn(v_ed_kn), Quantity::force_kn(v_rd_kn), "aluminium welded joint ULS", AnnexChoice::De)
    }
}
// #endregion 🔖Part1_6

/// 📐 Aluminium bending resistance M_c,Rd [kNm].
pub fn bending_resistance_knm(w_el_mm3: f64, f_0_2_mpa: f64, gamma_m: f64) -> f64 {
    w_el_mm3 * f_0_2_mpa / gamma_m / 1_000_000.0
}

/// 📐 Buckling resistance with χ factor.
pub fn buckling_resistance_kn(a_mm2: f64, f_0_2_mpa: f64, chi: f64, gamma_m: f64) -> f64 {
    chi * a_mm2 * f_0_2_mpa / gamma_m / 1000.0
}

/// 📋 Aluminium member check.
pub fn check_aluminium_member(n_ed_kn: f64, m_ed_knm: f64, a_mm2: f64, w_el_mm3: f64, alloy: part_1_1::Alloy, chi: f64, i_t_mm4: f64, l_cr_mm: f64) -> CheckReport {
    let gamma_m = na_de::GAMMA_M;
    let annex = AnnexChoice::De;
    let f_0_2 = alloy.f_0_2_mpa();
    let n_rd = a_mm2 * f_0_2 / gamma_m / 1000.0;
    let n_b_rd = buckling_resistance_kn(a_mm2, f_0_2, chi, gamma_m);
    let n_t_rd = part_1_1::torsional_buckling_resistance_kn(a_mm2, i_t_mm4, l_cr_mm, alloy, gamma_m, 70_000.0);
    let m_rd = part_1_1::m_c_rd_knm(w_el_mm3, alloy, gamma_m);
    let mut report = CheckReport::default();
    report.push(part_1_1::check_cross_section(n_ed_kn, n_rd, annex));
    report.push(part_1_1::check_buckling(n_ed_kn, n_b_rd.min(n_t_rd), annex));
    report.push(part_1_1::check_bending(m_ed_knm, m_rd, annex));
    report
}

/// 📋 Full EN 1999 check across cross-section, buckling, bending, fire, fatigue, and welded joint parts.
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
) -> CheckReport {
    let mut report = check_aluminium_member(n_ed_kn, m_ed_knm, a_mm2, w_el_mm3, alloy, chi, i_t_mm4, l_cr_mm);
    let theta_cr = part_1_2::critical_temperature_c(alloy.f_0_2_mpa());
    report.push(part_1_2::check_fire_protection(theta_c, theta_cr));
    let delta_sigma_rd = part_1_3::fatigue_strength_mpa(delta_sigma_c, fatigue_m, n_cycles);
    report.push(part_1_3::check_fatigue(delta_sigma_ed, delta_sigma_rd));
    let a_w = part_1_6::weld_throat_area_mm2(weld_throat_mm, weld_length_mm);
    let v_weld_rd = part_1_6::weld_resistance_kn(a_w, alloy.f_u_mpa(), beta_w, na_de::GAMMA_M);
    report.push(part_1_6::check_welded_joint(v_weld_ed_kn, v_weld_rd));
    report
}

// #region 🔖Session
use norm_core::{NormFamily, NormFamilyId, NormHost, SetDocumentOp};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub n_ed_kn: f64,
    pub m_ed_knm: f64,
    pub a_mm2: f64,
    pub w_el_mm3: f64,
    pub alloy: String,
    pub chi: f64,
    pub i_t_mm4: f64,
    pub l_cr_mm: f64,
    pub theta_c: f64,
    pub delta_sigma_ed: f64,
    pub delta_sigma_c: f64,
    pub fatigue_m: f64,
    pub n_cycles: f64,
    pub v_weld_ed_kn: f64,
    pub weld_throat_mm: f64,
    pub weld_length_mm: f64,
    pub beta_w: f64,
}

impl Default for Document {
    fn default() -> Self {
        Self {
            n_ed_kn: 80.0,
            m_ed_knm: 4.0,
            a_mm2: 1200.0,
            w_el_mm3: 24_000.0,
            alloy: "aw6060t6".into(),
            chi: 0.85,
            i_t_mm4: 5000.0,
            l_cr_mm: 3000.0,
            theta_c: 200.0,
            delta_sigma_ed: 45.0,
            delta_sigma_c: 71.0,
            fatigue_m: 8.0,
            n_cycles: 500_000.0,
            v_weld_ed_kn: 25.0,
            weld_throat_mm: 4.0,
            weld_length_mm: 120.0,
            beta_w: 0.63,
        }
    }
}

pub type Op = SetDocumentOp<Document>;
pub type Host = NormHost<En1999Family>;

fn parse_alloy(value: &str) -> part_1_1::Alloy {
    match value.to_ascii_lowercase().as_str() {
        "aw6082t6" => part_1_1::Alloy::Aw6082T6,
        _ => part_1_1::Alloy::Aw6060T6,
    }
}

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
    )
}

pub struct En1999Family;

impl NormFamily for En1999Family {
    type Document = Document;
    type Op = Op;

    fn family_id() -> NormFamilyId {
        NormFamilyId::En1999
    }

    fn evaluate(document: &Document) -> CheckReport {
        evaluate(document)
    }
}
// #endregion 🔖Session

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alloy_6060_t6_m_c_rd() {
        let alloy = part_1_1::Alloy::Aw6060T6;
        let w_el = 24_000.0;
        let m_rd = part_1_1::m_c_rd_knm(w_el, alloy, na_de::GAMMA_M);
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
    fn fatigue_cutoff_at_2e6() {
        let strength = part_1_3::fatigue_strength_mpa(80.0, 8.0, 2_000_000.0);
        assert!((strength - 0.0).abs() < 1e-9);
        let below = part_1_3::fatigue_strength_mpa(80.0, 8.0, 1_000_000.0);
        assert!(below > 0.0);
    }

    #[test]
    fn hollow_section_torsion_constant() {
        let i_t = part_1_5::rectangular_rhs_i_t_mm4(100.0, 50.0, 3.0);
        assert!(i_t > 0.0);
    }

    #[test]
    fn aluminium_member_e2e() {
        let alloy = part_1_1::Alloy::Aw6082T6;
        let i_t = part_1_5::rectangular_rhs_i_t_mm4(100.0, 50.0, 3.0);
        let report = check_aluminium_member(80.0, 12.0, 1200.0, 15_000.0, alloy, 0.8, i_t, 3000.0);
        assert_eq!(report.checks.len(), 3);
    }

    #[test]
    fn weld_resistance_worked() {
        let a_w = part_1_6::weld_throat_area_mm2(4.0, 120.0);
        assert!((a_w - 480.0).abs() < 1e-9);
        let v_rd = part_1_6::weld_resistance_kn(a_w, 215.0, 0.63, na_de::GAMMA_M);
        assert!((v_rd - 148.92).abs() < 1.0);
    }

    #[test]
    fn fire_critical_temperature_6060() {
        let theta_cr = part_1_2::critical_temperature_c(190.0);
        assert!((theta_cr - 246.0).abs() < 0.1);
        let k_theta = part_1_2::strength_reduction_factor(300.0, theta_cr);
        assert!(k_theta < 1.0);
    }

    #[test]
    fn full_aluminium_worked_example() {
        let alloy = part_1_1::Alloy::Aw6060T6;
        let i_t = part_1_5::rectangular_rhs_i_t_mm4(100.0, 50.0, 3.0);
        let report = check_full_aluminium(80.0, 4.0, 1200.0, 24_000.0, alloy, 0.85, i_t, 3000.0, 200.0, 45.0, 71.0, 8.0, 500_000.0, 25.0, 4.0, 120.0, 0.63);
        assert_eq!(report.checks.len(), 6);
        assert!(report.checks[4].utilization < 1.0);
    }

    #[test]
    fn evaluate_runs_all_parts() {
        let report = evaluate(&Document::default());
        assert_eq!(report.checks.len(), 6);
    }
}
