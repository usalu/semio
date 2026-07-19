//! 🌋 EN 1998 design of structures for earthquake resistance.

use norm_core::{AnnexChoice, CheckReport, CheckResult, ClauseId, Quantity};
use serde::{Deserialize, Serialize};

// #region 🔖NaDe
pub mod na_de {
    pub use norm_en_1990::na_de::NaDe;

    /// 🌋 German seismic zone per DIN EN 1998-1/NA.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SeismicZone {
        Zone0,
        Zone1,
        Zone2,
        Zone3,
    }

    impl SeismicZone {
        pub fn as_u8(self) -> u8 {
            match self {
                Self::Zone0 => 0,
                Self::Zone1 => 1,
                Self::Zone2 => 2,
                Self::Zone3 => 3,
            }
        }

        pub fn a_g(self) -> f64 {
            match self {
                Self::Zone0 => 0.0,
                Self::Zone1 => 0.08,
                Self::Zone2 => 0.15,
                Self::Zone3 => 0.24,
            }
        }
    }

    /// 🪨 Ground type per EN 1998-1 Table 3.1 (DE NA).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GroundType {
        A,
        B,
        C,
        D,
        E,
    }

    impl GroundType {
        pub fn spectrum_params(self) -> (f64, f64, f64, f64) {
            match self {
                Self::A => (0.05, 0.25, 0.8, 1.0),
                Self::B => (0.15, 0.4, 2.0, 1.0),
                Self::C => (0.20, 0.6, 2.0, 1.15),
                Self::D => (0.25, 0.8, 2.0, 1.35),
                Self::E => (0.35, 1.2, 2.0, 1.4),
            }
        }
    }

    pub fn peak_ground_acceleration(zone: SeismicZone) -> f64 {
        zone.a_g()
    }
}
// #endregion 🔖NaDe

// #region 🔖Part1
pub mod part_1 {
    use super::*;

    /// 🏗️ Structural system behaviour factor q per EN 1998-1 Table 6.1.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum StructuralSystem {
        MomentFrameDch,
        MomentFrameDcm,
        MomentFrameDcl,
        ShearWall,
        BracedFrame,
        InvertedPendulum,
        DualSystem,
    }

    impl StructuralSystem {
        pub fn q(self) -> f64 {
            match self {
                Self::MomentFrameDch => 4.0,
                Self::MomentFrameDcm => 3.3,
                Self::MomentFrameDcl => 2.0,
                Self::ShearWall => 3.0,
                Self::BracedFrame => 2.5,
                Self::InvertedPendulum => 1.5,
                Self::DualSystem => 4.0,
            }
        }
    }

    /// 📊 Importance factor γ_I per EN 1998-1 Table 4.3.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ImportanceClass {
        Cc1,
        Cc2,
        Cc3,
        Cc4,
    }

    impl ImportanceClass {
        pub fn gamma_i(self) -> f64 {
            match self {
                Self::Cc1 => 0.8,
                Self::Cc2 => 1.0,
                Self::Cc3 => 1.2,
                Self::Cc4 => 1.4,
            }
        }
    }

    /// 📈 Elastic response spectrum Type 1 horizontal [g] per EN 1998-1 §3.2.2.
    pub fn elastic_response_spectrum_type1(a_g: f64, s: f64, tb: f64, tc: f64, td: f64, t: f64) -> f64 {
        let eta = 1.0;
        if t <= tb {
            a_g * s * (1.0 + t / tb * (2.5 * eta - 1.0))
        } else if t <= tc {
            a_g * s * 2.5 * eta
        } else if t <= td {
            a_g * s * 2.5 * eta * tc / t
        } else {
            a_g * s * 2.5 * eta * tc * td / (t * t)
        }
    }

    /// 📉 Design spectrum Sd(T) = S_e(T) · γ_I / q [g].
    pub fn design_spectrum_sd(s_e: f64, gamma_i: f64, q: f64) -> f64 {
        s_e * gamma_i / q
    }

    /// 🌊 Base shear V_b = S_e(T1) · m · γ_I / q [kN] with mass in tonnes.
    pub fn base_shear_kn(s_e: f64, mass_t: f64, gamma_i: f64, q: f64) -> f64 {
        s_e * mass_t * 9.81 * gamma_i / q
    }

    /// 🌊 Base shear from design spectrum S_d(T1) [kN].
    pub fn base_shear_from_design_kn(s_d: f64, mass_t: f64) -> f64 {
        s_d * mass_t * 9.81
    }

    /// 🔁 Redundancy factor ρ per EN 1998-1 §4.2.5.
    pub fn redundancy_factor(multiple_resisting_systems: bool) -> f64 {
        if multiple_resisting_systems {
            1.0
        } else {
            1.3
        }
    }

    /// 📐 Interstorey drift limit with ρ per EN 1998-1 §4.3.3.4 [mm].
    pub fn drift_limit_mm(height_m: f64, rho: f64, ductility: DuctilityClass, nu: f64) -> f64 {
        let theta = match ductility {
            DuctilityClass::Dch => 0.01,
            DuctilityClass::Dcm => 0.007,
            DuctilityClass::Dcl => 0.005,
        };
        nu * rho * theta * height_m * 1000.0
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DuctilityClass {
        Dch,
        Dcm,
        Dcl,
    }

    pub fn check_drift(drift_mm: f64, limit_mm: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1998-1", "§4.3", "4.3.3"), Quantity::length_m(drift_mm / 1000.0), Quantity::length_m(limit_mm / 1000.0), "interstorey drift SLS", annex)
    }

    pub fn check_base_shear(v_ed_kn: f64, v_rd_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1998-1", "§4.3", "4.3.4"), Quantity::force_kn(v_ed_kn), Quantity::force_kn(v_rd_kn), "seismic base shear ULS", annex)
    }
}
// #endregion 🔖Part1

// #region 🔖Part2
pub mod part_2 {
    use super::*;

    /// 🌉 Isolated bridge design spectrum reduction factor q_isol.
    pub fn isolation_reduction_factor(period_ratio: f64) -> f64 {
        (period_ratio * period_ratio).max(1.0)
    }

    /// 🌉 Design spectrum for isolated bridge deck [g].
    pub fn isolated_spectrum_sd(s_e: f64, gamma_i: f64, q_isol: f64) -> f64 {
        s_e * gamma_i / q_isol
    }

    /// 🌉 Bearing displacement check limit [mm].
    pub fn bearing_displacement_limit_mm(d_max_mm: f64) -> f64 {
        d_max_mm
    }

    pub fn check_bridge_seismic(v_ed: f64, v_rd: f64) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1998-2", "§5", "5.3"), Quantity::force_kn(v_ed), Quantity::force_kn(v_rd), "bridge seismic shear", AnnexChoice::En)
    }

    pub fn check_isolation_bearing(d_ed_mm: f64, d_rd_mm: f64) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1998-2", "§6", "6.5"), Quantity::length_m(d_ed_mm / 1000.0), Quantity::length_m(d_rd_mm / 1000.0), "isolation bearing displacement", AnnexChoice::En)
    }
}
// #endregion 🔖Part2

// #region 🔖Part3
pub mod part_3 {
    use super::*;

    /// 🏺 Impulsive period T_i [s] for circular silo per EN 1998-4.
    pub fn impulsive_period_s(height_m: f64, radius_m: f64) -> f64 {
        0.1 * (height_m / radius_m).sqrt()
    }

    /// 🏺 Convective period T_c [s] for circular silo.
    pub fn convective_period_s(radius_m: f64) -> f64 {
        2.0 * (radius_m / 9.81).sqrt()
    }

    /// 🏺 Impulsive mass ratio μ_i.
    pub fn impulsive_mass_ratio(h_over_r: f64) -> f64 {
        (0.45 * h_over_r / (1.0 + 0.75 * h_over_r)).clamp(0.1, 0.85)
    }

    /// 🏺 Convective mass ratio μ_c.
    pub fn convective_mass_ratio(h_over_r: f64) -> f64 {
        (0.55 / (1.0 + 0.75 * h_over_r)).clamp(0.05, 0.75)
    }

    /// 🏺 Combined silo base shear via SRSS [kN].
    pub fn silo_base_shear_kn(v_i_kn: f64, v_c_kn: f64) -> f64 {
        (v_i_kn * v_i_kn + v_c_kn * v_c_kn).sqrt()
    }

    pub fn check_silo_wall(n_ed_kn: f64, n_rd_kn: f64) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1998-4", "§7", "7.2"), Quantity::force_kn(n_ed_kn), Quantity::force_kn(n_rd_kn), "silo wall seismic", AnnexChoice::En)
    }

    pub fn check_silo_anchor(v_ed_kn: f64, v_rd_kn: f64) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1998-4", "§7", "7.4"), Quantity::force_kn(v_ed_kn), Quantity::force_kn(v_rd_kn), "silo anchorage", AnnexChoice::En)
    }
}
// #endregion 🔖Part3

// #region 🔖Part4
pub mod part_4 {
    use super::*;

    /// 🗼 Along-wind base overturning moment [kNm] per EN 1998-4 §7.6.
    pub fn along_wind_overturning_knm(rho_air: f64, v_crit_m_s: f64, height_m: f64, diameter_m: f64, c_d: f64) -> f64 {
        let q_z = 0.5 * rho_air * v_crit_m_s * v_crit_m_s / 1000.0;
        q_z * c_d * diameter_m * height_m * height_m / 2.0
    }

    /// 🗼 Critical wind speed for vortex shedding [m/s].
    pub fn critical_wind_speed_m_s(strouhal: f64, frequency_hz: f64, diameter_m: f64) -> f64 {
        strouhal * frequency_hz * diameter_m
    }

    /// 🗼 First-mode natural frequency [Hz] for cantilever tower.
    pub fn tower_frequency_hz(e_i_pa: f64, i_m4: f64, mass_kg_m: f64, height_m: f64) -> f64 {
        let lambda = 1.875;
        let omega = lambda * lambda * (e_i_pa * i_m4 / (mass_kg_m * height_m.powi(4))).sqrt();
        omega / (2.0 * std::f64::consts::PI)
    }

    pub fn check_tower_overturning(m_ed_knm: f64, m_rd_knm: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1998-4", "§7.6", "7.6.2"),
            Quantity::new(norm_core::QuantityKind::Moment, m_ed_knm * 1_000_000.0),
            Quantity::new(norm_core::QuantityKind::Moment, m_rd_knm * 1_000_000.0),
            "tower along-wind overturning",
            AnnexChoice::En,
        )
    }
}
// #endregion 🔖Part4

// #region 🔖Part5
pub mod part_5 {
    use super::*;

    /// 🏔️ Seismic coefficient k_h for dam body per EN 1998-5.
    pub fn dam_seismic_coefficient(a_g: f64, s: f64, gamma_i: f64) -> f64 {
        0.5 * a_g * s * gamma_i
    }

    /// 🏔️ Horizontal seismic force on dam [kN].
    pub fn dam_seismic_force_kn(k_h: f64, mass_t: f64) -> f64 {
        k_h * mass_t * 9.81
    }

    /// 🏔️ Mononobe-Okabe active thrust increment [kN/m].
    pub fn mononobe_okabe_thrust_kn_m(gamma_soil_kn_m3: f64, height_m: f64, phi_deg: f64, k_h: f64) -> f64 {
        let phi = phi_deg.to_radians();
        let ka = ((1.0 - k_h) / (1.0 + k_h) * (1.0 - phi.sin()) / (1.0 + phi.sin())).tan().powi(2);
        0.5 * gamma_soil_kn_m3 * height_m * height_m * ka
    }

    pub fn check_dam_stability(s_ed: f64, s_rd: f64) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1998-5", "§6", "6.2"), Quantity::new(norm_core::QuantityKind::Dimensionless, s_ed), Quantity::new(norm_core::QuantityKind::Dimensionless, s_rd), "dam seismic stability", AnnexChoice::En)
    }
}
// #endregion 🔖Part5

// #region 🔖Part6
pub mod part_6 {
    use super::*;

    /// 🧱 Foundation stiffness ratio r = K_f / K_s per EN 1998-5.
    pub fn stiffness_ratio(k_foundation: f64, k_soil: f64) -> f64 {
        k_foundation / k_soil
    }

    /// 🧱 Radiation damping ratio ξ for shallow foundation.
    pub fn radiation_damping(ratio: f64) -> f64 {
        (0.05 + 0.1 * ratio / (1.0 + ratio)).clamp(0.05, 0.20)
    }

    /// 🧱 Bearing capacity reduction factor under seismic per EN 1998-5 §6.5.
    pub fn bearing_reduction_factor(a_g: f64) -> f64 {
        (1.0 - 1.5 * a_g).max(0.5)
    }

    /// 🧱 Seismic bearing pressure check [kPa].
    pub fn seismic_bearing_pressure_kpa(v_seismic_kn: f64, area_m2: f64) -> f64 {
        v_seismic_kn / area_m2
    }

    pub fn check_foundation_bearing(p_ed_kpa: f64, p_rd_kpa: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1998-5", "§6.5", "6.5.2"),
            Quantity::new(norm_core::QuantityKind::Pressure, p_ed_kpa * 1000.0),
            Quantity::new(norm_core::QuantityKind::Pressure, p_rd_kpa * 1000.0),
            "foundation seismic bearing",
            AnnexChoice::De,
        )
    }

    pub fn check_foundation_sliding(h_ed_kn: f64, h_rd_kn: f64) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1998-5", "§6.5", "6.5.3"), Quantity::force_kn(h_ed_kn), Quantity::force_kn(h_rd_kn), "foundation seismic sliding", AnnexChoice::De)
    }
}
// #endregion 🔖Part6

/// 📋 Building seismic check (DE NA zone parameters).
pub fn check_building_seismic(
    zone: na_de::SeismicZone,
    ground: na_de::GroundType,
    importance: part_1::ImportanceClass,
    system: part_1::StructuralSystem,
    t1_s: f64,
    mass_t: f64,
    v_rd_kn: f64,
    drift_mm: f64,
    height_m: f64,
    multiple_resisting_systems: bool,
) -> CheckReport {
    let annex = AnnexChoice::De;
    let a_g = zone.a_g();
    let (tb, tc, td, s) = ground.spectrum_params();
    let gamma_i = importance.gamma_i();
    let q = system.q();
    let s_e = part_1::elastic_response_spectrum_type1(a_g, s, tb, tc, td, t1_s);
    let s_d = part_1::design_spectrum_sd(s_e, gamma_i, q);
    let v_b = part_1::base_shear_kn(s_e, mass_t, gamma_i, q);
    let _ = s_d;
    let rho = part_1::redundancy_factor(multiple_resisting_systems);
    let drift_limit = part_1::drift_limit_mm(height_m, rho, part_1::DuctilityClass::Dcm, 1.0);
    let mut report = CheckReport::default();
    report.push(part_1::check_base_shear(v_b, v_rd_kn, annex));
    report.push(part_1::check_drift(drift_mm, drift_limit, annex));
    report
}

// #region 🔖Session
use norm_core::{NormFamily, NormFamilyId, NormHost, SetDocumentOp};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub seismic_zone: u8,
    pub ground_type: String,
    pub importance_class: String,
    pub structural_system: String,
    pub t1_s: f64,
    pub mass_t: f64,
    pub v_rd_kn: f64,
    pub drift_mm: f64,
    pub height_m: f64,
    pub multiple_resisting_systems: bool,
}

impl Default for Document {
    fn default() -> Self {
        Self { seismic_zone: 2, ground_type: "b".into(), importance_class: "cc2".into(), structural_system: "moment_frame_dch".into(), t1_s: 0.3, mass_t: 500.0, v_rd_kn: 800.0, drift_mm: 20.0, height_m: 12.0, multiple_resisting_systems: true }
    }
}

pub type Op = SetDocumentOp<Document>;
pub type Host = NormHost<En1998Family>;

fn parse_seismic_zone(value: u8) -> na_de::SeismicZone {
    match value {
        0 => na_de::SeismicZone::Zone0,
        1 => na_de::SeismicZone::Zone1,
        3 => na_de::SeismicZone::Zone3,
        _ => na_de::SeismicZone::Zone2,
    }
}

fn parse_ground_type(value: &str) -> na_de::GroundType {
    match value.to_ascii_lowercase().as_str() {
        "a" => na_de::GroundType::A,
        "c" => na_de::GroundType::C,
        "d" => na_de::GroundType::D,
        "e" => na_de::GroundType::E,
        _ => na_de::GroundType::B,
    }
}

fn parse_importance(value: &str) -> part_1::ImportanceClass {
    match value.to_ascii_lowercase().as_str() {
        "cc1" => part_1::ImportanceClass::Cc1,
        "cc3" => part_1::ImportanceClass::Cc3,
        "cc4" => part_1::ImportanceClass::Cc4,
        _ => part_1::ImportanceClass::Cc2,
    }
}

fn parse_structural_system(value: &str) -> part_1::StructuralSystem {
    match value.to_ascii_lowercase().as_str() {
        "moment_frame_dcm" => part_1::StructuralSystem::MomentFrameDcm,
        "moment_frame_dcl" => part_1::StructuralSystem::MomentFrameDcl,
        "shear_wall" => part_1::StructuralSystem::ShearWall,
        "braced_frame" => part_1::StructuralSystem::BracedFrame,
        "inverted_pendulum" => part_1::StructuralSystem::InvertedPendulum,
        "dual_system" => part_1::StructuralSystem::DualSystem,
        _ => part_1::StructuralSystem::MomentFrameDch,
    }
}

pub fn evaluate(document: &Document) -> CheckReport {
    check_building_seismic(
        parse_seismic_zone(document.seismic_zone),
        parse_ground_type(&document.ground_type),
        parse_importance(&document.importance_class),
        parse_structural_system(&document.structural_system),
        document.t1_s,
        document.mass_t,
        document.v_rd_kn,
        document.drift_mm,
        document.height_m,
        document.multiple_resisting_systems,
    )
}

pub struct En1998Family;

impl NormFamily for En1998Family {
    type Document = Document;
    type Op = Op;

    fn family_id() -> NormFamilyId {
        NormFamilyId::En1998
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
    fn zone2_spectrum_sd_at_t1() {
        let a_g = na_de::SeismicZone::Zone2.a_g();
        let (tb, tc, td, s) = na_de::GroundType::B.spectrum_params();
        let s_e = part_1::elastic_response_spectrum_type1(a_g, s, tb, tc, td, 0.3);
        assert!((s_e - 0.375).abs() < 1e-9);
        let sd = part_1::design_spectrum_sd(s_e, 1.0, 1.0);
        assert!((sd - 0.375).abs() < 1e-9);
    }

    #[test]
    fn base_shear_uses_design_spectrum() {
        let a_g = 0.15;
        let (tb, tc, td, s) = na_de::GroundType::B.spectrum_params();
        let s_e = part_1::elastic_response_spectrum_type1(a_g, s, tb, tc, td, 0.3);
        let gamma_i = part_1::ImportanceClass::Cc2.gamma_i();
        let q = part_1::StructuralSystem::MomentFrameDch.q();
        let s_d = part_1::design_spectrum_sd(s_e, gamma_i, q);
        let v_b = part_1::base_shear_kn(s_e, 100.0, gamma_i, q);
        assert!(v_b > 0.0);
        let expected = s_e * 100.0 * 9.81 * gamma_i / q;
        assert!((v_b - expected).abs() < 1e-6);
        assert!((part_1::base_shear_from_design_kn(s_d, 100.0) - v_b).abs() < 1e-6);
    }

    #[test]
    fn drift_rho_limit() {
        let rho = part_1::redundancy_factor(false);
        assert!((rho - 1.3).abs() < 1e-9);
        let limit = part_1::drift_limit_mm(12.0, rho, part_1::DuctilityClass::Dcm, 1.0);
        assert!((limit - 109.2).abs() < 0.1);
    }

    #[test]
    fn building_seismic_e2e() {
        let report = check_building_seismic(na_de::SeismicZone::Zone2, na_de::GroundType::B, part_1::ImportanceClass::Cc2, part_1::StructuralSystem::MomentFrameDch, 0.3, 500.0, 800.0, 20.0, 12.0, true);
        assert_eq!(report.checks.len(), 2);
    }

    #[test]
    fn silo_impulsive_convective() {
        let h = 10.0;
        let r = 5.0;
        let t_i = part_3::impulsive_period_s(h, r);
        let t_c = part_3::convective_period_s(r);
        assert!(t_i < t_c);
        let v = part_3::silo_base_shear_kn(200.0, 150.0);
        assert!((v - 250.0).abs() < 1e-6);
    }

    #[test]
    fn bridge_isolation_distinct() {
        let q_isol = part_2::isolation_reduction_factor(2.0);
        assert!((q_isol - 4.0).abs() < 1e-9);
    }
}
