//! ⚙️ EN 1997 app — headless compute (constitutional: engine).
//! 📚️ Models the classic (pre-2024) Eurocode 7 generation only: EN 1997-1 (general design rules, including piles) + EN 1997-2 (ground investigation and testing); the second-generation EN 1997-3 does not apply here.

use crate::artifacts::en1997::En1997Snapshot;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::document::{AnnexChoice, CheckReport, CheckResult, ClauseId, Quantity};

pub mod na_de {
    pub use crate::artifacts::en1990::engine::na_de::NaDe;

    /// 🇩️🇪️ Partial factor on cohesion per DIN EN 1997-1/NA.
    pub fn gamma_c() -> f64 {
        1.4
    }

    /// 🇩️🇪️ Partial factor on angle of shearing resistance per DIN EN 1997-1/NA.
    pub fn gamma_phi() -> f64 {
        1.25
    }

    /// 🇩️🇪️ Partial factor on soil weight per DIN EN 1997-1/NA.
    pub fn gamma_gamma() -> f64 {
        1.0
    }
}

/// ⚖️ Design approach per EN 1997-1 §2.4.7.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DesignApproach {
    Da1Str,
    Da1Geo,
    Da2,
    Da3,
}

/// 🇩️🇪️🇺️ Annex-resolved partial factor set for a design approach; DE selects DA2* (combined STR/GEO verification) where EN keeps standard DA2.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AnnexParams {
    pub gamma_g: f64,
    pub gamma_q: f64,
    pub gamma_r_v: f64,
    pub gamma_r_h: f64,
    pub gamma_c: f64,
    pub gamma_phi: f64,
    pub gamma_gamma: f64,
    pub gamma_b: f64,
    pub gamma_s: f64,
}

impl DesignApproach {
    /// 🏷️ Approach label as resolved under the national annex (DE's `Da2` is DA2*, EN's is DA2).
    pub fn label(self, annex: AnnexChoice) -> &'static str {
        match (self, annex) {
            (Self::Da2, AnnexChoice::De) => "DA2*",
            (Self::Da2, AnnexChoice::En) => "DA2",
            (Self::Da1Str, _) => "DA1-C1",
            (Self::Da1Geo, _) => "DA1-C2",
            (Self::Da3, _) => "DA3",
        }
    }

    /// ⚖️ Resolve the full partial factor set for this approach under the given national annex per EN 1997-1 Annex A / DIN EN 1997-1/NA.
    pub fn annex_params(self, annex: AnnexChoice) -> AnnexParams {
        match (self, annex) {
            (Self::Da2, AnnexChoice::De) => AnnexParams { gamma_g: 1.35, gamma_q: 1.5, gamma_r_v: 1.4, gamma_r_h: 1.1, gamma_c: 1.0, gamma_phi: 1.0, gamma_gamma: 1.0, gamma_b: 1.1, gamma_s: 1.1 },
            (Self::Da2, AnnexChoice::En) => AnnexParams { gamma_g: 1.35, gamma_q: 1.5, gamma_r_v: 1.0, gamma_r_h: 1.0, gamma_c: 1.0, gamma_phi: 1.0, gamma_gamma: 1.0, gamma_b: 1.1, gamma_s: 1.1 },
            (Self::Da1Str, _) => AnnexParams { gamma_g: 1.35, gamma_q: 1.5, gamma_r_v: 1.0, gamma_r_h: 1.0, gamma_c: na_de::gamma_c(), gamma_phi: na_de::gamma_phi(), gamma_gamma: na_de::gamma_gamma(), gamma_b: 1.0, gamma_s: 1.0 },
            (Self::Da1Geo, _) => AnnexParams { gamma_g: 1.0, gamma_q: 1.3, gamma_r_v: 1.4, gamma_r_h: 1.1, gamma_c: 1.0, gamma_phi: 1.0, gamma_gamma: 1.0, gamma_b: 1.1, gamma_s: 1.1 },
            (Self::Da3, _) => AnnexParams { gamma_g: 1.35, gamma_q: 1.5, gamma_r_v: 1.0, gamma_r_h: 1.0, gamma_c: na_de::gamma_c(), gamma_phi: na_de::gamma_phi(), gamma_gamma: na_de::gamma_gamma(), gamma_b: 1.15, gamma_s: 1.15 },
        }
    }
}

// #region 🔖️Part1
pub mod part_1 {
    use super::*;

    /// 📐️ Bearing capacity factor N_q (Meyerhof).
    pub fn bearing_factor_n_q(phi_deg: f64) -> f64 {
        let phi_rad = phi_deg.to_radians();
        (std::f64::consts::PI * phi_rad.tan()).exp() * ((45.0 + 0.5 * phi_deg).to_radians().tan()).powi(2)
    }

    /// 📐️ Bearing capacity factor N_c (Meyerhof) with depth correction.
    pub fn bearing_factor_n_c(phi_deg: f64, d_f_m: f64, b_m: f64) -> f64 {
        let phi_rad = phi_deg.to_radians();
        if phi_deg <= 0.0 {
            return 5.14;
        }
        let n_q = bearing_factor_n_q(phi_deg);
        let s_c = if b_m > 0.0 { 1.0 + 0.2 * (d_f_m / b_m) * (20.0 - phi_deg).max(0.0) / 20.0 } else { 1.0 };
        (n_q - 1.0) / phi_rad.tan() * s_c
    }

    /// 📐️ Bearing capacity factor N_γ (Meyerhof).
    pub fn bearing_factor_n_gamma(phi_deg: f64, d_f_m: f64, b_m: f64) -> f64 {
        let phi_rad = phi_deg.to_radians();
        let n_q = bearing_factor_n_q(phi_deg);
        let s_gamma = if b_m > 0.0 { 1.0 + 0.1 * (d_f_m / b_m) * (20.0 - phi_deg).max(0.0) / 20.0 } else { 1.0 };
        2.0 * (n_q + 1.0) * phi_rad.tan() * s_gamma
    }

    /// 📐️ Ultimate bearing capacity q_ult [kPa] (Meyerhof).
    pub fn ultimate_bearing_capacity_kpa(phi_deg: f64, c_kpa: f64, gamma_kn_m3: f64, b_m: f64, d_f_m: f64) -> f64 {
        let n_c = bearing_factor_n_c(phi_deg, d_f_m, b_m);
        let n_q = bearing_factor_n_q(phi_deg);
        let n_gamma = bearing_factor_n_gamma(phi_deg, d_f_m, b_m);
        c_kpa * n_c + gamma_kn_m3 * d_f_m * n_q + 0.5 * gamma_kn_m3 * b_m * n_gamma
    }

    pub fn design_bearing_capacity_kpa(phi_deg: f64, c_kpa: f64, gamma_kn_m3: f64, b_m: f64, d_f_m: f64, approach: DesignApproach, annex: AnnexChoice) -> f64 {
        let p = approach.annex_params(annex);
        let phi_d = (phi_deg.to_radians() / p.gamma_phi).atan().to_degrees();
        let c_d = c_kpa / p.gamma_c;
        let gamma_d = gamma_kn_m3 / p.gamma_gamma;
        ultimate_bearing_capacity_kpa(phi_d, c_d, gamma_d, b_m, d_f_m) / p.gamma_r_v
    }

    pub fn bearing_resistance_kn(a_m2: f64, q_d_kpa: f64) -> f64 {
        a_m2 * q_d_kpa
    }

    /// 📐️ Sliding resistance [kPa]: (c_d + σ·tan(φ_d)) / γ_R;h.
    pub fn sliding_resistance_kpa(phi_deg: f64, c_kpa: f64, sigma_kpa: f64, approach: DesignApproach, annex: AnnexChoice) -> f64 {
        let p = approach.annex_params(annex);
        let phi_d = (phi_deg.to_radians() / p.gamma_phi).atan();
        let c_d = c_kpa / p.gamma_c;
        (c_d + sigma_kpa * phi_d.tan()) / p.gamma_r_h
    }

    pub fn sliding_resistance_kn(phi_deg: f64, c_kpa: f64, sigma_kpa: f64, area_m2: f64, approach: DesignApproach, annex: AnnexChoice) -> f64 {
        sliding_resistance_kpa(phi_deg, c_kpa, sigma_kpa, approach, annex) * area_m2
    }

    /// 📐️ Elastic settlement [mm] (Boussinesq simplified).
    pub fn elastic_settlement_mm(e_s_mpa: f64, nu: f64, b_m: f64, q_kpa: f64) -> f64 {
        let i_f = 0.88;
        let q_mpa = q_kpa / 1000.0;
        i_f * q_mpa * b_m * (1.0 - nu * nu) / e_s_mpa * 1000.0
    }

    // #region 🔖️Piles
    /// 📐️ Shaft resistance R_s = α_s · π · D · q_s · L [kN] per EN 1997-1 §7.6.2 (ground-test-derived α_s and q_s from EN 1997-2 investigation).
    pub fn shaft_resistance_kn(alpha_s: f64, d_m: f64, q_s_kpa: f64, l_m: f64) -> f64 {
        alpha_s * std::f64::consts::PI * d_m * q_s_kpa * l_m
    }

    /// 📐️ Base resistance R_b = q_b · A_b [kN] per EN 1997-1 §7.6.2.
    pub fn base_resistance_kn(q_b_kpa: f64, a_base_m2: f64) -> f64 {
        q_b_kpa * a_base_m2
    }

    /// 📊️ Correlation factors ξ₃ (on mean) / ξ₄ (on min) per EN 1997-1 Annex A Table A.10, by number of test profiles n.
    pub fn pile_correlation_factors(n_profiles: u32) -> (f64, f64) {
        match n_profiles {
            0 | 1 => (1.40, 1.40),
            2 => (1.35, 1.27),
            3 | 4 => {
                let t = (n_profiles - 2) as f64 / 3.0;
                (1.35 + t * (1.30 - 1.35), 1.27 + t * (1.15 - 1.27))
            }
            _ => (1.30, 1.15),
        }
    }

    /// 📐️ Characteristic pile resistance R_k = min(mean(R_cal)/ξ₃, min(R_cal)/ξ₄) per EN 1997-1 §7.6.2.2.
    pub fn pile_characteristic_resistance_kn(mean_r_cal_kn: f64, min_r_cal_kn: f64, n_profiles: u32) -> f64 {
        let (xi_3, xi_4) = pile_correlation_factors(n_profiles);
        (mean_r_cal_kn / xi_3).min(min_r_cal_kn / xi_4)
    }

    /// 📐️ Design pile compressive resistance R_c,d = R_b,k/γ_b + R_s,k/γ_s per EN 1997-1 §7.6.2.
    pub fn pile_design_resistance_kn(r_b_k_kn: f64, r_s_k_kn: f64, approach: DesignApproach, annex: AnnexChoice) -> f64 {
        let p = approach.annex_params(annex);
        r_b_k_kn / p.gamma_b + r_s_k_kn / p.gamma_s
    }
    // #endregion 🔖️Piles

    pub fn check_bearing(v_ed_kn: f64, r_d_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1997-1", "§6.5", "6.5"), Quantity::force_kn(v_ed_kn), Quantity::force_kn(r_d_kn), "bearing resistance ULS", annex)
    }

    pub fn check_sliding(h_ed_kn: f64, r_d_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1997-1", "§6.5", "6.5.3"), Quantity::force_kn(h_ed_kn), Quantity::force_kn(r_d_kn), "sliding resistance ULS", annex)
    }

    pub fn check_settlement(s_mm: f64, limit_mm: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1997-1", "§6.6", "6.6"), Quantity::length_m(s_mm / 1000.0), Quantity::length_m(limit_mm / 1000.0), "settlement SLS", annex)
    }

    /// ✅️ Pile axial compressive resistance check per EN 1997-1 §7.6.2.
    pub fn check_pile_axial(n_ed_kn: f64, r_c_d_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1997-1", "§7", "7.6.2"), Quantity::force_kn(n_ed_kn), Quantity::force_kn(r_c_d_kn), "pile axial compressive resistance ULS", annex)
    }
}
// #endregion 🔖️Part1

// #region 🔖️Part2
pub mod part_2 {
    use super::{AnnexChoice, CheckResult, ClauseId, Quantity};

    /// 📐️ Effective friction angle φ′ [deg] from CPT cone resistance q_c and effective overburden σ′_v0, per EN 1997-2 Annex D (Robertson & Campanella 1983).
    pub fn phi_from_cpt_deg(q_c_kpa: f64, sigma_v0_kpa: f64) -> f64 {
        let ratio = (q_c_kpa / sigma_v0_kpa.max(1.0)).max(1.0);
        (0.38 * ratio.log10() + 0.1).atan().to_degrees()
    }

    /// 📐️ Effective friction angle φ′ [deg] from SPT blow count N, per EN 1997-2 Annex D (Peck–Hanson–Thornburn correlation).
    pub fn phi_from_spt_deg(n_spt: f64) -> f64 {
        27.1 + 0.3 * n_spt - 0.00054 * n_spt * n_spt
    }

    /// 📐️ Minimum ground investigation depth [m] per EN 1997-2 §2.4.2: z ≥ 3·b.
    pub fn min_investigation_depth_m(b_m: f64) -> f64 {
        3.0 * b_m
    }

    /// ✅️ Ground investigation depth adequacy check per EN 1997-2 §2.4.2.
    pub fn check_investigation_depth(z_investigated_m: f64, b_m: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_minimum(ClauseId::new("EN 1997-2", "§2.4", "2.4.2"), Quantity::length_m(z_investigated_m), Quantity::length_m(min_investigation_depth_m(b_m)), "minimum ground investigation depth", annex)
    }
}
// #endregion 🔖️Part2

/// 📋️ Shallow foundation check.
#[allow(clippy::too_many_arguments, reason = "one argument per parameter the published clause formula itself names; bundling them into a struct would break the 1:1 reading against the standard")]
pub fn check_shallow_foundation(
    v_ed_kn: f64,
    h_ed_kn: f64,
    footing_area_m2: f64,
    phi_deg: f64,
    c_kpa: f64,
    gamma_kn_m3: f64,
    b_m: f64,
    d_f_m: f64,
    e_s_mpa: f64,
    nu: f64,
    approach: DesignApproach,
    annex: AnnexChoice,
    settlement_limit_mm: f64,
) -> CheckReport {
    let q_d = part_1::design_bearing_capacity_kpa(phi_deg, c_kpa, gamma_kn_m3, b_m, d_f_m, approach, annex);
    let r_d = part_1::bearing_resistance_kn(footing_area_m2, q_d);
    let sigma = v_ed_kn / footing_area_m2;
    let r_sliding = part_1::sliding_resistance_kn(phi_deg, c_kpa, sigma, footing_area_m2, approach, annex);
    let settlement = part_1::elastic_settlement_mm(e_s_mpa, nu, b_m, sigma);
    let mut report = CheckReport::default();
    report.push(part_1::check_bearing(v_ed_kn, r_d, annex));
    report.push(part_1::check_sliding(h_ed_kn, r_sliding, annex));
    report.push(part_1::check_settlement(settlement, settlement_limit_mm, annex));
    report
}

/// 📋️ Full EN 1997 check across bearing, sliding, settlement, pile axial (part 1), and ground investigation adequacy (part 2).
#[allow(clippy::too_many_arguments, reason = "one argument per parameter the published clause formula itself names; bundling them into a struct would break the 1:1 reading against the standard")]
pub fn check_full_geotechnical(
    v_ed_kn: f64,
    h_ed_kn: f64,
    footing_area_m2: f64,
    phi_deg: f64,
    c_kpa: f64,
    gamma_kn_m3: f64,
    b_m: f64,
    d_f_m: f64,
    e_s_mpa: f64,
    nu: f64,
    approach: DesignApproach,
    annex: AnnexChoice,
    settlement_limit_mm: f64,
    n_pile_ed_kn: f64,
    alpha_s: f64,
    pile_d_m: f64,
    q_s_kpa: f64,
    pile_l_m: f64,
    q_b_kpa: f64,
    pile_base_area_m2: f64,
    pile_n_profiles: u32,
    z_investigated_m: f64,
) -> CheckReport {
    let mut report = check_shallow_foundation(v_ed_kn, h_ed_kn, footing_area_m2, phi_deg, c_kpa, gamma_kn_m3, b_m, d_f_m, e_s_mpa, nu, approach, annex, settlement_limit_mm);
    let r_s_cal = part_1::shaft_resistance_kn(alpha_s, pile_d_m, q_s_kpa, pile_l_m);
    let r_b_cal = part_1::base_resistance_kn(q_b_kpa, pile_base_area_m2);
    let r_s_k = part_1::pile_characteristic_resistance_kn(r_s_cal, r_s_cal, pile_n_profiles);
    let r_b_k = part_1::pile_characteristic_resistance_kn(r_b_cal, r_b_cal, pile_n_profiles);
    let r_c_d = part_1::pile_design_resistance_kn(r_b_k, r_s_k, approach, annex);
    report.push(part_1::check_pile_axial(n_pile_ed_kn, r_c_d, annex));
    report.push(part_2::check_investigation_depth(z_investigated_m, b_m, annex));
    report
}

// #region 🔖️Session
fn parse_design_approach(value: &str) -> DesignApproach {
    match value.to_ascii_lowercase().as_str() {
        "da1geo" => DesignApproach::Da1Geo,
        "da2" => DesignApproach::Da2,
        "da3" => DesignApproach::Da3,
        _ => DesignApproach::Da1Str,
    }
}

/// 🧮️ Headless per-document evaluation — the `NormFamily::evaluate` body for `En1997Family` (defined
/// in the sibling `op` crate, which depends on this `engine` crate to call it).
pub fn evaluate(document: &En1997Snapshot) -> CheckReport {
    check_full_geotechnical(
        document.v_ed_kn,
        document.h_ed_kn,
        document.footing_area_m2,
        document.phi_deg,
        document.c_kpa,
        document.gamma_kn_m3,
        document.b_m,
        document.d_f_m,
        document.e_s_mpa,
        document.nu,
        parse_design_approach(&document.design_approach),
        document.annex,
        document.settlement_limit_mm,
        document.n_pile_ed_kn,
        document.alpha_s,
        document.pile_d_m,
        document.q_s_kpa,
        document.pile_l_m,
        document.q_b_kpa,
        document.pile_base_area_m2,
        document.pile_n_profiles,
        document.z_investigated_m,
    )
}
// #endregion 🔖️Session

// #region 🔖️Session
/// 🧩️ EN 1997's `NormFamily` binding — ties this artifact's `En1997Snapshot` to the `evaluate` above for the
/// headless `NormHost` session every norm app drives.
pub struct En1997Family;

impl crate::document::NormFamily for En1997Family {
    type Document = crate::artifacts::en1997::En1997Snapshot;
    type Mutation = crate::artifacts::en1997::mutations::En1997Mutation;

    fn family_id() -> crate::document::NormFamilyId {
        crate::document::NormFamilyId::En1997
    }

    fn evaluate(document: &Self::Document) -> crate::document::CheckReport {
        evaluate(document)
    }
}


//#region 🔖️ArtifactEngine
/// ⚙️ UI-independent En1997 artifact engine — owns the full artifact; `snapshot()` is persisted only.
pub struct En1997Engine {
    artifact: crate::artifacts::en1997::schema::En1997Artifact,
    snapshot: crate::artifacts::en1997::En1997Snapshot,
}

impl En1997Engine {
    pub fn new(snapshot: crate::artifacts::en1997::En1997Snapshot) -> Self {
        let artifact = crate::artifacts::en1997::schema::En1997Artifact::from_snapshot(snapshot.clone());
        Self { artifact, snapshot }
    }

    pub fn into_snapshot(self) -> crate::artifacts::en1997::En1997Snapshot {
        self.snapshot
    }
}
//#endregion 🔖️ArtifactEngine

pub type Host = crate::document::NormHost<En1997Family>;
// #endregion 🔖️Session

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::CheckStatus;

    #[test]
    fn bearing_factor_n_c_phi30() {
        let n_c = part_1::bearing_factor_n_c(30.0, 1.5, 2.0);
        assert!((n_c - 30.1).abs() < 0.5);
    }

    #[test]
    fn shallow_foundation_e2e() {
        let report = check_shallow_foundation(500.0, 80.0, 2.0, 30.0, 0.0, 18.0, 2.0, 1.5, 30_000.0, 0.3, DesignApproach::Da1Str, AnnexChoice::De, 25.0);
        assert!(!report.checks.is_empty());
    }

    #[test]
    fn pile_design_resistance_worked() {
        let r_s = part_1::shaft_resistance_kn(0.7, 0.6, 80.0, 12.0);
        assert!((r_s - 1266.69).abs() < 1.0);
        let r_b = part_1::base_resistance_kn(2500.0, 0.28);
        assert!((r_b - 700.0).abs() < 0.1);
        let r_c_d = part_1::pile_design_resistance_kn(r_b, r_s, DesignApproach::Da2, AnnexChoice::De);
        let expected = r_b / 1.1 + r_s / 1.1;
        assert!((r_c_d - expected).abs() < 1e-6);
    }

    #[test]
    fn pile_correlation_factors_n2_matches_annex_a_table() {
        let (xi_3, xi_4) = part_1::pile_correlation_factors(2);
        assert!((xi_3 - 1.35).abs() < 1e-9);
        assert!((xi_4 - 1.27).abs() < 1e-9);
        let mean_r_cal_kn = 1000.0;
        let min_r_cal_kn = 900.0;
        let r_k = part_1::pile_characteristic_resistance_kn(mean_r_cal_kn, min_r_cal_kn, 2);
        let expected = (mean_r_cal_kn / 1.35_f64).min(min_r_cal_kn / 1.27_f64);
        assert!((r_k - expected).abs() < 1e-6);
    }

    #[test]
    fn pile_correlation_factors_boundary_cases() {
        assert_eq!(part_1::pile_correlation_factors(1), (1.40, 1.40));
        assert_eq!(part_1::pile_correlation_factors(5), (1.30, 1.15));
        assert_eq!(part_1::pile_correlation_factors(9), (1.30, 1.15));
    }

    #[test]
    fn investigation_depth_check_pass_and_fail() {
        let b_m = 2.0;
        let min_depth = part_2::min_investigation_depth_m(b_m);
        assert!((min_depth - 6.0).abs() < 1e-9);
        let pass = part_2::check_investigation_depth(8.0, b_m, AnnexChoice::De);
        assert_eq!(pass.status, CheckStatus::Pass);
        let fail = part_2::check_investigation_depth(4.0, b_m, AnnexChoice::De);
        assert_eq!(fail.status, CheckStatus::Fail);
    }

    #[test]
    fn phi_from_cpt_worked_example() {
        let q_c_kpa = 15_000.0;
        let sigma_v0_kpa = 100.0;
        let phi = part_2::phi_from_cpt_deg(q_c_kpa, sigma_v0_kpa);
        let expected = (0.38 * (q_c_kpa / sigma_v0_kpa).log10() + 0.1).atan().to_degrees();
        assert!((phi - expected).abs() < 1e-9);
        assert!(phi > 25.0 && phi < 45.0);
    }

    #[test]
    fn phi_from_spt_worked_example() {
        let phi = part_2::phi_from_spt_deg(20.0);
        let expected = 27.1 + 0.3 * 20.0 - 0.00054 * 20.0 * 20.0;
        assert!((phi - expected).abs() < 1e-9);
    }

    #[test]
    fn da2_star_de_diverges_from_da2_en_on_same_footing() {
        let q_d_de = part_1::design_bearing_capacity_kpa(30.0, 0.0, 18.0, 2.0, 1.5, DesignApproach::Da2, AnnexChoice::De);
        let q_d_en = part_1::design_bearing_capacity_kpa(30.0, 0.0, 18.0, 2.0, 1.5, DesignApproach::Da2, AnnexChoice::En);
        assert!(q_d_de < q_d_en);
        assert!((q_d_en / q_d_de - 1.4).abs() < 1e-9);
        assert_eq!(DesignApproach::Da2.label(AnnexChoice::De), "DA2*");
        assert_eq!(DesignApproach::Da2.label(AnnexChoice::En), "DA2");
    }

    #[test]
    fn full_geotechnical_worked_example() {
        let report = check_full_geotechnical(500.0, 80.0, 2.0, 30.0, 0.0, 18.0, 2.0, 1.5, 30_000.0, 0.3, DesignApproach::Da1Str, AnnexChoice::De, 25.0, 800.0, 0.7, 0.6, 80.0, 12.0, 2500.0, 0.28, 1, 8.0);
        assert_eq!(report.checks.len(), 5);
        assert!(report.checks[3].utilization < 1.0);
        assert_eq!(report.checks[4].status, CheckStatus::Pass);
    }

    #[test]
    fn evaluate_runs_all_parts() {
        let report = evaluate(&En1997Snapshot::default());
        assert_eq!(report.checks.len(), 5);
    }
}
//#endregion 🧪️Tests


/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    register_artifact_schema();
    dsl::register_language(dsl::LanguageSpec {
        id: "en1997.document",
        extension: Some("en1997"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::en1996::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::en1996::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::en1996::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::en1996::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("en1997.document"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "en1997.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::en1996::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::en1996::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::en1996::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::en1996::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("en1997.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "en1997.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::en1996::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::en1996::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("en1997.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "en1997.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::en1996::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::en1996::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("en1997.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "en1997.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::en1996::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::en1996::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("en1997.spr"),
    });
}

//#region 🔖️SchemaRegistry
use std::sync::{Mutex, OnceLock};

/// 📌️ Registers the twenty handcrafted schema leaves for `s.norm.en1997`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::en1997::schema::en1997_artifact_schema_descriptor());
}

/// 💡️ Registers `s.norm.en1997.inference`'s facet leaves into the OS-wide inference catalog —
/// sibling to `register_artifact_schema()` above (separate registry, ticket
/// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
pub fn register_artifact_inferences() {
    ::schema::register_artifact_inference_descriptor(crate::artifacts::en1997::standards::v1::subsets::any::schema::inferences::en1997_artifact_inference_descriptor());
}
//#endregion 🔖️SchemaRegistry

//#region 🔖️IoFacet
pub fn register_io() {
    crate::artifacts::en1997::io_registry::register();
}
//#endregion 🔖️IoFacet
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, composer_entry_of};
    use crate::artifacts::en1997::standards::v1::subsets::any::schema::En1997Composer as En1997AnyComposer;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![
            composer_entry_of::<En1997AnyComposer>(),
        ]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
