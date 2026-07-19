//! 🌍 EN 1997 geotechnical design.

use norm_core::{AnnexChoice, CheckReport, CheckResult, ClauseId, Quantity};
use serde::{Deserialize, Serialize};

pub mod na_de {
    pub use norm_en_1990::na_de::NaDe;

    /// 🇩🇪 Partial factor on cohesion per DIN EN 1997-1/NA.
    pub fn gamma_c() -> f64 {
        1.4
    }

    /// 🇩🇪 Partial factor on angle of shearing resistance per DIN EN 1997-1/NA.
    pub fn gamma_phi() -> f64 {
        1.25
    }

    /// 🇩🇪 Partial factor on soil weight per DIN EN 1997-1/NA.
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

impl DesignApproach {
    pub fn gamma_r(self) -> f64 {
        match self {
            Self::Da1Str | Self::Da2 => 1.0,
            Self::Da1Geo | Self::Da3 => 1.4,
        }
    }

    pub fn gamma_set(self) -> (f64, f64, f64) {
        match self {
            Self::Da1Str => (na_de::gamma_c(), na_de::gamma_phi(), na_de::gamma_gamma()),
            Self::Da1Geo => (1.0, 1.0, 1.0),
            Self::Da2 => (1.0, 1.0, 1.0),
            Self::Da3 => (na_de::gamma_c(), na_de::gamma_phi(), na_de::gamma_gamma()),
        }
    }
}

// #region 🔖Part1
pub mod part_1 {
    use super::*;

    /// 📐 Bearing capacity factor N_q (Meyerhof).
    pub fn bearing_factor_n_q(phi_deg: f64) -> f64 {
        let phi_rad = phi_deg.to_radians();
        let n_q = (std::f64::consts::PI * phi_rad.tan()).exp()
            * ((45.0 + 0.5 * phi_deg).to_radians().tan()).powi(2);
        n_q
    }

    /// 📐 Bearing capacity factor N_c (Meyerhof) with depth correction.
    pub fn bearing_factor_n_c(phi_deg: f64, d_f_m: f64, b_m: f64) -> f64 {
        let phi_rad = phi_deg.to_radians();
        if phi_deg <= 0.0 {
            return 5.14;
        }
        let n_q = bearing_factor_n_q(phi_deg);
        let s_c = if b_m > 0.0 {
            1.0 + 0.2 * (d_f_m / b_m) * (20.0 - phi_deg).max(0.0) / 20.0
        } else {
            1.0
        };
        (n_q - 1.0) / phi_rad.tan() * s_c
    }

    /// 📐 Bearing capacity factor N_γ (Meyerhof).
    pub fn bearing_factor_n_gamma(phi_deg: f64, d_f_m: f64, b_m: f64) -> f64 {
        let phi_rad = phi_deg.to_radians();
        let n_q = bearing_factor_n_q(phi_deg);
        let s_gamma = if b_m > 0.0 {
            1.0 + 0.1 * (d_f_m / b_m) * (20.0 - phi_deg).max(0.0) / 20.0
        } else {
            1.0
        };
        2.0 * (n_q + 1.0) * phi_rad.tan() * s_gamma
    }

    /// 📐 Ultimate bearing capacity q_ult [kPa] (Meyerhof).
    pub fn ultimate_bearing_capacity_kpa(
        phi_deg: f64,
        c_kpa: f64,
        gamma_kn_m3: f64,
        b_m: f64,
        d_f_m: f64,
    ) -> f64 {
        let n_c = bearing_factor_n_c(phi_deg, d_f_m, b_m);
        let n_q = bearing_factor_n_q(phi_deg);
        let n_gamma = bearing_factor_n_gamma(phi_deg, d_f_m, b_m);
        c_kpa * n_c + gamma_kn_m3 * d_f_m * n_q + 0.5 * gamma_kn_m3 * b_m * n_gamma
    }

    pub fn design_bearing_capacity_kpa(
        phi_deg: f64,
        c_kpa: f64,
        gamma_kn_m3: f64,
        b_m: f64,
        d_f_m: f64,
        approach: DesignApproach,
    ) -> f64 {
        let (gamma_c, gamma_phi, gamma_gamma) = approach.gamma_set();
        let phi_d = (phi_deg.to_radians() / gamma_phi).atan().to_degrees();
        let c_d = c_kpa / gamma_c;
        let gamma_d = gamma_kn_m3 / gamma_gamma;
        ultimate_bearing_capacity_kpa(phi_d, c_d, gamma_d, b_m, d_f_m) / approach.gamma_r()
    }

    pub fn bearing_resistance_kn(a_m2: f64, q_ult_kpa: f64, gamma_r: f64) -> f64 {
        a_m2 * q_ult_kpa / gamma_r
    }

    /// 📐 Sliding resistance [kPa]: c_d + σ·tan(φ_d).
    pub fn sliding_resistance_kpa(
        phi_deg: f64,
        c_kpa: f64,
        sigma_kpa: f64,
        approach: DesignApproach,
    ) -> f64 {
        let (gamma_c, gamma_phi, _) = approach.gamma_set();
        let phi_d = (phi_deg.to_radians() / gamma_phi).atan();
        let c_d = c_kpa / gamma_c;
        c_d + sigma_kpa * phi_d.tan()
    }

    pub fn sliding_resistance_kn(
        phi_deg: f64,
        c_kpa: f64,
        sigma_kpa: f64,
        area_m2: f64,
        approach: DesignApproach,
    ) -> f64 {
        sliding_resistance_kpa(phi_deg, c_kpa, sigma_kpa, approach) * area_m2
    }

    /// 📐 Elastic settlement [mm] (Boussinesq simplified).
    pub fn elastic_settlement_mm(e_s_mpa: f64, nu: f64, b_m: f64, q_kpa: f64) -> f64 {
        let i_f = 0.88;
        let q_mpa = q_kpa / 1000.0;
        i_f * q_mpa * b_m * (1.0 - nu * nu) / e_s_mpa * 1000.0
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

    pub fn pile_axial_resistance_kn(
        r_b_kn: f64,
        r_s_kn: f64,
        gamma_r: f64,
    ) -> f64 {
        (r_b_kn + r_s_kn) / gamma_r
    }

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
    phi_deg: f64,
    c_kpa: f64,
    gamma_kn_m3: f64,
    b_m: f64,
    d_f_m: f64,
    e_s_mpa: f64,
    nu: f64,
    approach: DesignApproach,
    settlement_limit_mm: f64,
) -> CheckReport {
    let annex = AnnexChoice::De;
    let q_d = part_1::design_bearing_capacity_kpa(phi_deg, c_kpa, gamma_kn_m3, b_m, d_f_m, approach);
    let r_d = part_1::bearing_resistance_kn(footing_area_m2, q_d, 1.0);
    let sigma = v_ed_kn / footing_area_m2;
    let r_sliding = part_1::sliding_resistance_kn(phi_deg, c_kpa, sigma, footing_area_m2, approach);
    let settlement = part_1::elastic_settlement_mm(e_s_mpa, nu, b_m, sigma);
    let mut report = CheckReport::default();
    report.push(part_1::check_bearing(v_ed_kn, r_d, annex));
    report.push(part_1::check_sliding(h_ed_kn, r_sliding, annex));
    report.push(part_1::check_settlement(settlement, settlement_limit_mm, annex));
    report
}

// #region 🔖Session
use norm_core::{NormFamily, NormFamilyId, NormHost, SetDocumentOp};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub v_ed_kn: f64,
    pub h_ed_kn: f64,
    pub footing_area_m2: f64,
    pub phi_deg: f64,
    pub c_kpa: f64,
    pub gamma_kn_m3: f64,
    pub b_m: f64,
    pub d_f_m: f64,
    pub e_s_mpa: f64,
    pub nu: f64,
    pub design_approach: String,
    pub settlement_limit_mm: f64,
}

impl Default for Document {
    fn default() -> Self {
        Self {
            v_ed_kn: 500.0,
            h_ed_kn: 80.0,
            footing_area_m2: 2.0,
            phi_deg: 30.0,
            c_kpa: 0.0,
            gamma_kn_m3: 18.0,
            b_m: 2.0,
            d_f_m: 1.5,
            e_s_mpa: 30_000.0,
            nu: 0.3,
            design_approach: "da1str".into(),
            settlement_limit_mm: 25.0,
        }
    }
}

pub type Op = SetDocumentOp<Document>;
pub type Host = NormHost<En1997Family>;

fn parse_design_approach(value: &str) -> DesignApproach {
    match value.to_ascii_lowercase().as_str() {
        "da1geo" => DesignApproach::Da1Geo,
        "da2" => DesignApproach::Da2,
        "da3" => DesignApproach::Da3,
        _ => DesignApproach::Da1Str,
    }
}

pub fn evaluate(document: &Document) -> CheckReport {
    check_shallow_foundation(
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
        document.settlement_limit_mm,
    )
}

pub struct En1997Family;

impl NormFamily for En1997Family {
    type Document = Document;
    type Op = Op;

    fn family_id() -> NormFamilyId {
        NormFamilyId::En1997
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
    fn bearing_factor_n_c_phi30() {
        let n_c = part_1::bearing_factor_n_c(30.0, 1.5, 2.0);
        assert!((n_c - 30.1).abs() < 0.5);
    }

    #[test]
    fn shallow_foundation_e2e() {
        let report = check_shallow_foundation(
            500.0,
            80.0,
            2.0,
            30.0,
            0.0,
            18.0,
            2.0,
            1.5,
            30_000.0,
            0.3,
            DesignApproach::Da1Str,
            25.0,
        );
        assert!(!report.checks.is_empty());
    }
}
