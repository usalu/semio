//! 🧱 EN 1996 design of masonry structures.

use norm_core::{AnnexChoice, CheckReport, CheckResult, ClauseId, Quantity};
use serde::{Deserialize, Serialize};

pub mod na_de {
    pub use norm_en_1990::na_de::NaDe;

    /// 🇩🇪 Partial factor γ_M per DIN EN 1996-1-1/NA.
    pub fn gamma_m() -> f64 {
        2.0
    }
}

/// 🧱 Masonry unit type per EN 1996-1-1.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MasonryUnit {
    Clay,
    CalciumSilicate,
    Aac,
}

impl MasonryUnit {
    pub fn label(self) -> &'static str {
        match self {
            Self::Clay => "clay",
            Self::CalciumSilicate => "calcium silicate",
            Self::Aac => "AAC",
        }
    }
}

// #region 🔖Part1_1
pub mod part_1_1 {
    use super::*;

    pub fn design_strength_mpa(f_k_mpa: f64, gamma_m: f64) -> f64 {
        f_k_mpa / gamma_m
    }

    pub fn flexural_resistance_knm(z_mm3: f64, f_xd_mpa: f64) -> f64 {
        z_mm3 * f_xd_mpa / 1_000_000.0
    }

    pub fn compression_resistance_kn(a_mm2: f64, f_d_mpa: f64) -> f64 {
        a_mm2 * f_d_mpa / 1000.0
    }

    pub fn shear_design_strength_mpa(f_vk_mpa: f64, gamma_m: f64) -> f64 {
        f_vk_mpa / gamma_m
    }

    pub fn shear_resistance_kn(a_mm2: f64, f_vd_mpa: f64) -> f64 {
        a_mm2 * f_vd_mpa / 1000.0
    }

    pub fn sliding_resistance_kn(mu: f64, n_ed_kn: f64, f_vd_mpa: f64, a_mm2: f64) -> f64 {
        mu * n_ed_kn + a_mm2 * f_vd_mpa / 1000.0
    }

    pub fn check_flexure(m_ed: f64, m_rd: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1996-1-1", "§6.2", "6.2"), Quantity::new(norm_core::QuantityKind::Moment, m_ed * 1_000_000.0), Quantity::new(norm_core::QuantityKind::Moment, m_rd * 1_000_000.0), "masonry flexure ULS", annex)
    }

    pub fn check_compression(sigma_ed_mpa: f64, f_d_mpa: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1996-1-1", "§6.1.2", "6.1"), Quantity::stress_mpa(sigma_ed_mpa), Quantity::stress_mpa(f_d_mpa), "masonry compression ULS", annex)
    }

    pub fn check_shear(v_ed_kn: f64, v_rd_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1996-1-1", "§6.2.3", "6.2"), Quantity::force_kn(v_ed_kn), Quantity::force_kn(v_rd_kn), "masonry shear ULS", annex)
    }

    pub fn check_sliding(h_ed_kn: f64, h_rd_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1996-1-1", "§6.2.4", "6.2"), Quantity::force_kn(h_ed_kn), Quantity::force_kn(h_rd_kn), "masonry sliding ULS", annex)
    }
}
// #endregion 🔖Part1_1

// #region 🔖Part1_2
pub mod part_1_2 {
    use super::*;

    /// 🔥 Minimum fire wall thickness [mm] per EN 1996-1-2 Table 5.1 (simplified).
    pub fn required_wall_thickness_mm(fire_resistance_min: u32, unit: MasonryUnit) -> f64 {
        let base = match fire_resistance_min {
            30 => 60.0,
            60 => 90.0,
            90 => 120.0,
            120 => 150.0,
            180 => 200.0,
            240 => 250.0,
            _ => 90.0,
        };
        match unit {
            MasonryUnit::Clay => base,
            MasonryUnit::CalciumSilicate => base * 1.1,
            MasonryUnit::Aac => base * 1.25,
        }
    }

    pub fn check_fire_wall(thickness_mm: f64, required_mm: f64) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1996-1-2", "§4", "4.1"), Quantity::length_m(thickness_mm / 1000.0), Quantity::length_m(required_mm / 1000.0), "masonry fire wall thickness", AnnexChoice::De)
    }
}
// #endregion 🔖Part1_2

// #region 🔖Part2
pub mod part_2 {
    use super::*;

    pub fn lintel_shear_resistance_kn(a_mm2: f64, f_vk_mpa: f64, gamma_m: f64) -> f64 {
        let f_vd = part_1_1::shear_design_strength_mpa(f_vk_mpa, gamma_m);
        part_1_1::shear_resistance_kn(a_mm2, f_vd)
    }

    pub fn check_lintel_shear(v_ed_kn: f64, v_rd_kn: f64) -> CheckResult {
        part_1_1::check_shear(v_ed_kn, v_rd_kn, AnnexChoice::En)
    }
}
// #endregion 🔖Part2

// #region 🔖Part3
pub mod part_3 {
    use super::*;

    /// 🌍 Active earth pressure coefficient K_a per Rankine (c=0).
    pub fn active_earth_pressure_coefficient(phi_deg: f64) -> f64 {
        let phi_rad = phi_deg.to_radians();
        ((1.0 - phi_rad.sin()) / (1.0 + phi_rad.sin())).powi(2)
    }

    /// 🌍 Resultant active earth pressure [kN/m] on retaining wall.
    pub fn active_earth_pressure_kn_m(gamma_soil_kn_m3: f64, h_m: f64, phi_deg: f64) -> f64 {
        let ka = active_earth_pressure_coefficient(phi_deg);
        0.5 * gamma_soil_kn_m3 * h_m * h_m * ka
    }

    /// 🌍 Overturning moment [kNm/m] at wall base from active earth pressure.
    pub fn retaining_wall_overturning_moment_knm(gamma_soil_kn_m3: f64, h_m: f64, phi_deg: f64) -> f64 {
        let ka = active_earth_pressure_coefficient(phi_deg);
        gamma_soil_kn_m3 * h_m.powi(3) * ka / 6.0
    }

    pub fn check_retaining_wall(m_ed: f64, m_rd: f64) -> CheckResult {
        part_1_1::check_flexure(m_ed, m_rd, AnnexChoice::En)
    }
}
// #endregion 🔖Part3

/// 📋 Masonry wall under vertical load.
pub fn check_masonry_wall(n_ed_kn: f64, area_mm2: f64, f_k_mpa: f64, gamma_m: f64) -> CheckReport {
    let sigma = n_ed_kn * 1000.0 / area_mm2;
    let f_d = part_1_1::design_strength_mpa(f_k_mpa, gamma_m);
    let mut report = CheckReport::default();
    report.push(part_1_1::check_compression(sigma, f_d, AnnexChoice::De));
    report
}

fn parse_masonry_unit(value: &str) -> MasonryUnit {
    match value.to_ascii_lowercase().as_str() {
        "calcium_silicate" | "calcium silicate" => MasonryUnit::CalciumSilicate,
        "aac" => MasonryUnit::Aac,
        _ => MasonryUnit::Clay,
    }
}

/// 📋 Full EN 1996 check across flexure, compression, shear, sliding, fire wall, and retaining parts.
pub fn check_full_masonry(
    m_ed_knm: f64,
    n_ed_kn: f64,
    v_ed_kn: f64,
    h_ed_kn: f64,
    z_mm3: f64,
    area_mm2: f64,
    shear_area_mm2: f64,
    f_k_mpa: f64,
    f_vk_mpa: f64,
    gamma_m: f64,
    mu: f64,
    wall_thickness_mm: f64,
    fire_resistance_min: u32,
    unit: MasonryUnit,
    gamma_soil_kn_m3: f64,
    wall_height_m: f64,
    phi_deg: f64,
    m_rd_knm: f64,
) -> CheckReport {
    let f_d = part_1_1::design_strength_mpa(f_k_mpa, gamma_m);
    let f_vd = part_1_1::shear_design_strength_mpa(f_vk_mpa, gamma_m);
    let sigma = n_ed_kn * 1000.0 / area_mm2;
    let m_rd_flex = part_1_1::flexural_resistance_knm(z_mm3, f_d);
    let v_rd = part_1_1::shear_resistance_kn(shear_area_mm2, f_vd);
    let h_rd = part_1_1::sliding_resistance_kn(mu, n_ed_kn, f_vd, shear_area_mm2);
    let annex = AnnexChoice::De;
    let mut report = CheckReport::default();
    report.push(part_1_1::check_flexure(m_ed_knm, m_rd_flex, annex));
    report.push(part_1_1::check_compression(sigma, f_d, annex));
    report.push(part_1_1::check_shear(v_ed_kn, v_rd, annex));
    report.push(part_1_1::check_sliding(h_ed_kn, h_rd, annex));
    let required_fire = part_1_2::required_wall_thickness_mm(fire_resistance_min, unit);
    report.push(part_1_2::check_fire_wall(wall_thickness_mm, required_fire));
    let m_overturn = part_3::retaining_wall_overturning_moment_knm(gamma_soil_kn_m3, wall_height_m, phi_deg);
    report.push(part_3::check_retaining_wall(m_overturn, m_rd_knm));
    report
}

// #region 🔖Session
use norm_core::{NormFamily, NormFamilyId, NormHost, SetDocumentOp};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub m_ed_knm: f64,
    pub n_ed_kn: f64,
    pub v_ed_kn: f64,
    pub h_ed_kn: f64,
    pub z_mm3: f64,
    pub area_mm2: f64,
    pub shear_area_mm2: f64,
    pub f_k_mpa: f64,
    pub f_vk_mpa: f64,
    pub gamma_m: f64,
    pub mu: f64,
    pub wall_thickness_mm: f64,
    pub fire_resistance_min: u32,
    pub unit: String,
    pub gamma_soil_kn_m3: f64,
    pub wall_height_m: f64,
    pub phi_deg: f64,
    pub m_rd_knm: f64,
}

impl Default for Document {
    fn default() -> Self {
        Self {
            m_ed_knm: 8.0,
            n_ed_kn: 200.0,
            v_ed_kn: 35.0,
            h_ed_kn: 20.0,
            z_mm3: 8_000_000.0,
            area_mm2: 500_000.0,
            shear_area_mm2: 300_000.0,
            f_k_mpa: 5.0,
            f_vk_mpa: 0.15,
            gamma_m: 2.0,
            mu: 0.4,
            wall_thickness_mm: 100.0,
            fire_resistance_min: 60,
            unit: "clay".into(),
            gamma_soil_kn_m3: 18.0,
            wall_height_m: 3.0,
            phi_deg: 30.0,
            m_rd_knm: 25.0,
        }
    }
}

pub type Op = SetDocumentOp<Document>;
pub type Host = NormHost<En1996Family>;

pub fn evaluate(document: &Document) -> CheckReport {
    check_full_masonry(
        document.m_ed_knm,
        document.n_ed_kn,
        document.v_ed_kn,
        document.h_ed_kn,
        document.z_mm3,
        document.area_mm2,
        document.shear_area_mm2,
        document.f_k_mpa,
        document.f_vk_mpa,
        document.gamma_m,
        document.mu,
        document.wall_thickness_mm,
        document.fire_resistance_min,
        parse_masonry_unit(&document.unit),
        document.gamma_soil_kn_m3,
        document.wall_height_m,
        document.phi_deg,
        document.m_rd_knm,
    )
}

pub struct En1996Family;

impl NormFamily for En1996Family {
    type Document = Document;
    type Op = Op;

    fn family_id() -> NormFamilyId {
        NormFamilyId::En1996
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
    fn masonry_wall_sigma_vs_fd() {
        let sigma = 200.0 * 1000.0 / 500_000.0;
        let f_d = part_1_1::design_strength_mpa(5.0, na_de::gamma_m());
        assert!((sigma - 0.4_f64).abs() < 1e-9);
        assert!((f_d - 2.5_f64).abs() < 1e-9);
        let report = check_masonry_wall(200.0, 500_000.0, 5.0, na_de::gamma_m());
        assert!(!report.checks.is_empty());
        assert!(report.checks[0].utilization < 1.0);
    }

    #[test]
    fn masonry_wall_e2e() {
        let report = check_masonry_wall(200.0, 500_000.0, 5.0, 2.0);
        assert!(!report.checks.is_empty());
    }

    #[test]
    fn active_earth_pressure_rankine() {
        let ka = part_3::active_earth_pressure_coefficient(30.0);
        assert!((ka - 0.111).abs() < 0.01);
        let p_a = part_3::active_earth_pressure_kn_m(18.0, 3.0, 30.0);
        assert!((p_a - 9.0).abs() < 1.0);
    }

    #[test]
    fn fire_wall_r60_clay() {
        let required = part_1_2::required_wall_thickness_mm(60, MasonryUnit::Clay);
        assert!((required - 90.0).abs() < 0.1);
    }

    #[test]
    fn full_masonry_worked_example() {
        let report = check_full_masonry(8.0, 200.0, 35.0, 20.0, 8_000_000.0, 500_000.0, 300_000.0, 5.0, 0.15, 2.0, 0.4, 100.0, 60, MasonryUnit::Clay, 18.0, 3.0, 30.0, 25.0);
        assert_eq!(report.checks.len(), 6);
        let m_overturn = part_3::retaining_wall_overturning_moment_knm(18.0, 3.0, 30.0);
        assert!((m_overturn - 9.0).abs() < 0.5);
    }

    #[test]
    fn evaluate_runs_all_parts() {
        let report = evaluate(&Document::default());
        assert_eq!(report.checks.len(), 6);
    }
}
