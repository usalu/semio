//! 🪵 EN 1995 design of timber structures.

use norm_core::{AnnexChoice, CheckReport, CheckResult, ClauseId, LoadDuration, Quantity};
use serde::{Deserialize, Serialize};

pub mod na_de {
    pub use norm_en_1990::na_de::NaDe;

    use super::{k_def, LoadDuration, ServiceClass};

    /// 🇩🇪 Partial factor γ_M per DIN EN 1995-1-1/NA.
    pub fn gamma_m() -> f64 {
        1.3
    }

    /// 🇩🇪 k_def per DIN EN 1995-1-1/NA Table NA.3.
    pub fn k_def_de(service: ServiceClass, duration: LoadDuration) -> f64 {
        k_def(service, duration)
    }
}

/// 🌡️ Service class per EN 1995-1-1 Table 3.1.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ServiceClass {
    Sc1,
    Sc2,
    Sc3,
}

// #region 🔖Kmod
/// 📊 k_mod per EN 1995-1-1 Table 3.1 (service class × load duration).
pub fn k_mod(service: ServiceClass, duration: LoadDuration) -> f64 {
    match (service, duration) {
        (ServiceClass::Sc1, LoadDuration::Permanent) => 0.6,
        (ServiceClass::Sc1, LoadDuration::Long) => 0.7,
        (ServiceClass::Sc1, LoadDuration::Medium) => 0.8,
        (ServiceClass::Sc1, LoadDuration::Short) => 0.9,
        (ServiceClass::Sc1, LoadDuration::Instantaneous) => 1.1,
        (ServiceClass::Sc2, LoadDuration::Permanent) => 0.6,
        (ServiceClass::Sc2, LoadDuration::Long) => 0.7,
        (ServiceClass::Sc2, LoadDuration::Medium) => 0.8,
        (ServiceClass::Sc2, LoadDuration::Short) => 0.9,
        (ServiceClass::Sc2, LoadDuration::Instantaneous) => 1.1,
        (ServiceClass::Sc3, LoadDuration::Permanent) => 0.5,
        (ServiceClass::Sc3, LoadDuration::Long) => 0.55,
        (ServiceClass::Sc3, LoadDuration::Medium) => 0.65,
        (ServiceClass::Sc3, LoadDuration::Short) => 0.7,
        (ServiceClass::Sc3, LoadDuration::Instantaneous) => 0.9,
    }
}

/// 📊 k_def per EN 1995-1-1 Table 3.2.
pub fn k_def(service: ServiceClass, duration: LoadDuration) -> f64 {
    match (service, duration) {
        (ServiceClass::Sc1, LoadDuration::Permanent) => 0.60,
        (ServiceClass::Sc1, LoadDuration::Long) => 0.70,
        (ServiceClass::Sc1, LoadDuration::Medium) => 0.80,
        (ServiceClass::Sc1, LoadDuration::Short) => 0.90,
        (ServiceClass::Sc1, LoadDuration::Instantaneous) => 2.00,
        (ServiceClass::Sc2, LoadDuration::Permanent) => 0.60,
        (ServiceClass::Sc2, LoadDuration::Long) => 0.70,
        (ServiceClass::Sc2, LoadDuration::Medium) => 0.80,
        (ServiceClass::Sc2, LoadDuration::Short) => 1.10,
        (ServiceClass::Sc2, LoadDuration::Instantaneous) => 1.75,
        (ServiceClass::Sc3, LoadDuration::Permanent) => 2.00,
        (ServiceClass::Sc3, LoadDuration::Long) => 2.25,
        (ServiceClass::Sc3, LoadDuration::Medium) => 2.50,
        (ServiceClass::Sc3, LoadDuration::Short) => 2.50,
        (ServiceClass::Sc3, LoadDuration::Instantaneous) => 3.00,
    }
}

/// 📐 Lateral torsional buckling factor k_crit per EN 1995-1-1 Eq. 6.25.
pub fn k_crit(lambda_rel_m: f64) -> f64 {
    if lambda_rel_m <= 0.75 {
        1.0
    } else if lambda_rel_m <= 1.4 {
        1.0 / (lambda_rel_m + (lambda_rel_m * lambda_rel_m - 0.75_f64.powi(2)).sqrt())
    } else {
        1.0 / (lambda_rel_m * lambda_rel_m)
    }
}

/// 📐 Relative slenderness λ_rel,m for lateral torsional buckling.
pub fn lambda_rel_m(w_eff_mm3: f64, f_m_k_mpa: f64, m_crit_knm: f64) -> f64 {
    (w_eff_mm3 * f_m_k_mpa / 1_000_000.0 / m_crit_knm).sqrt()
}
// #endregion 🔖Kmod

// #region 🔖Part1_1
pub mod part_1_1 {
    use super::*;

    pub fn bending_resistance_knm(w_mm3: f64, f_m_k_mpa: f64, k_mod: f64, k_crit: f64) -> f64 {
        k_mod * k_crit * w_mm3 * f_m_k_mpa / na_de::gamma_m() / 1_000_000.0
    }

    pub fn compression_resistance_kn(a_mm2: f64, f_c_0_k_mpa: f64, k_mod: f64) -> f64 {
        k_mod * a_mm2 * f_c_0_k_mpa / na_de::gamma_m() / 1000.0
    }

    pub fn connection_bearing_resistance_kn(
        a_ef_mm2: f64,
        f_v_k_mpa: f64,
        k_mod: f64,
    ) -> f64 {
        k_mod * a_ef_mm2 * f_v_k_mpa / na_de::gamma_m() / 1000.0
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

    pub fn check_connection_bearing(f_ed_kn: f64, f_rd_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1995-1-1", "§8.1.2", "8.1"),
            Quantity::force_kn(f_ed_kn),
            Quantity::force_kn(f_rd_kn),
            "timber connection bearing ULS",
            annex,
        )
    }
}
// #endregion 🔖Part1_1

// #region 🔖Part1_2
pub mod part_1_2 {
    use super::*;

    /// 🔥 One-dimensional charring rate β₀ [mm/min] per EN 1995-1-2 §4.2.2.
    pub const CHARRING_RATE_MM_MIN: f64 = 0.65;

    pub fn charred_depth_mm(fire_duration_min: f64) -> f64 {
        CHARRING_RATE_MM_MIN * fire_duration_min
    }

    pub fn residual_section_mm(original_mm: f64, charred_depth_mm: f64) -> f64 {
        (original_mm - 2.0 * charred_depth_mm).max(0.0)
    }

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
    use super::{part_1_1, AnnexChoice, CheckResult, LoadDuration, ServiceClass, k_mod};

    pub fn bridge_bending_resistance_knm(
        w_mm3: f64,
        f_m_k_mpa: f64,
        service: ServiceClass,
        duration: LoadDuration,
        k_crit: f64,
    ) -> f64 {
        part_1_1::bending_resistance_knm(w_mm3, f_m_k_mpa, k_mod(service, duration), k_crit)
    }

    pub fn check_bridge_timber(m_ed: f64, m_rd: f64) -> CheckResult {
        part_1_1::check_bending(m_ed, m_rd, AnnexChoice::En)
    }
}
// #endregion 🔖Part2

/// 📋 Glulam beam check with LTB.
pub fn check_glulam_beam(
    m_ed_knm: f64,
    n_ed_kn: f64,
    w_mm3: f64,
    a_mm2: f64,
    f_m_k: f64,
    f_c_0_k: f64,
    service: ServiceClass,
    duration: LoadDuration,
    m_crit_knm: f64,
) -> CheckReport {
    let km = k_mod(service, duration);
    let lambda = lambda_rel_m(w_mm3, f_m_k, m_crit_knm);
    let kc = k_crit(lambda);
    let m_rd = part_1_1::bending_resistance_knm(w_mm3, f_m_k, km, kc);
    let n_rd = part_1_1::compression_resistance_kn(a_mm2, f_c_0_k, km);
    let annex = AnnexChoice::De;
    let mut report = CheckReport::default();
    report.push(part_1_1::check_bending(m_ed_knm, m_rd, annex));
    report.push(part_1_1::check_compression(n_ed_kn, n_rd, annex));
    report
}

// #region 🔖Session
use norm_core::{NormFamily, NormFamilyId, NormHost, SetDocumentOp};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub m_ed_knm: f64,
    pub n_ed_kn: f64,
    pub w_mm3: f64,
    pub a_mm2: f64,
    pub f_m_k: f64,
    pub f_c_0_k: f64,
    pub service_class: String,
    pub load_duration: String,
    pub m_crit_knm: f64,
}

impl Default for Document {
    fn default() -> Self {
        Self {
            m_ed_knm: 25.0,
            n_ed_kn: 50.0,
            w_mm3: 1_000_000.0,
            a_mm2: 20_000.0,
            f_m_k: 24.0,
            f_c_0_k: 21.0,
            service_class: "sc1".into(),
            load_duration: "medium".into(),
            m_crit_knm: 80.0,
        }
    }
}

pub type Op = SetDocumentOp<Document>;
pub type Host = NormHost<En1995Family>;

fn parse_service_class(value: &str) -> ServiceClass {
    match value.to_ascii_lowercase().as_str() {
        "sc2" => ServiceClass::Sc2,
        "sc3" => ServiceClass::Sc3,
        _ => ServiceClass::Sc1,
    }
}

fn parse_load_duration(value: &str) -> LoadDuration {
    match value.to_ascii_lowercase().as_str() {
        "permanent" => LoadDuration::Permanent,
        "long" => LoadDuration::Long,
        "short" => LoadDuration::Short,
        "instantaneous" => LoadDuration::Instantaneous,
        _ => LoadDuration::Medium,
    }
}

pub fn evaluate(document: &Document) -> CheckReport {
    check_glulam_beam(
        document.m_ed_knm,
        document.n_ed_kn,
        document.w_mm3,
        document.a_mm2,
        document.f_m_k,
        document.f_c_0_k,
        parse_service_class(&document.service_class),
        parse_load_duration(&document.load_duration),
        document.m_crit_knm,
    )
}

pub struct En1995Family;

impl NormFamily for En1995Family {
    type Document = Document;
    type Op = Op;

    fn family_id() -> NormFamilyId {
        NormFamilyId::En1995
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
    fn k_mod_sc1_permanent() {
        assert!((k_mod(ServiceClass::Sc1, LoadDuration::Permanent) - 0.6).abs() < 1e-9);
    }

    #[test]
    fn glulam_ltb_reduces_bending_resistance() {
        let w = 1_000_000.0;
        let f_m_k = 24.0;
        let km = k_mod(ServiceClass::Sc1, LoadDuration::Medium);
        let m_crit = 30.0;
        let lambda = lambda_rel_m(w, f_m_k, m_crit);
        let kc = k_crit(lambda);
        assert!(kc < 1.0);
        let m_rd = part_1_1::bending_resistance_knm(w, f_m_k, km, kc);
        let m_rd_full = part_1_1::bending_resistance_knm(w, f_m_k, km, 1.0);
        assert!(m_rd < m_rd_full);
    }

    #[test]
    fn glulam_beam_e2e() {
        let report = check_glulam_beam(
            25.0,
            50.0,
            1_000_000.0,
            20_000.0,
            24.0,
            21.0,
            ServiceClass::Sc1,
            LoadDuration::Medium,
            80.0,
        );
        assert!(!report.checks.is_empty());
    }
}
