//! ⚙️ EN 1996 app — headless compute (constitutional: engine).
//!
//! 🔀️ `MasonryClass`/`part_2::ExposureClass`/`part_2::MortarClass` are En1996Snapshot field types, so they
//! live in the sibling `en1996` (rs) crate per the entity-schema mapping rule; this crate's own
//! `part_2` submodule holds only the pure-compute functions that were originally interleaved with
//! those entity enums, re-importing the types from `crate::artifacts::en1996::part_2`.

use crate::artifacts::en1996::{En1996Snapshot, MasonryClass};
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::document::{AnnexChoice, CheckReport, CheckResult, CheckStatus, ClauseId, DesignSituation, Quantity};
use serde::{Deserialize, Serialize};

pub mod na_de {
    pub use crate::artifacts::en1990::engine::na_de::NaDe;

    /// 🇩️🇪️ Partial factor γ_M per DIN EN 1996-1-1/NA (flat, independent of masonry class).
    pub fn gamma_m() -> f64 {
        super::AnnexParams { annex: crate::document::AnnexChoice::De, masonry_class: crate::artifacts::en1996::MasonryClass::default(), accidental: false }.gamma_m()
    }
}

/// 🧱️ Masonry unit type per EN 1996-1-1.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

// #region 🔖️Annex
/// ⚖️ Resolved national-annex parameters governing the masonry partial factor γ_M (EN 1996-1-1 §2.4.3 vs DIN EN 1996-1-1/NA).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnnexParams {
    pub annex: AnnexChoice,
    pub masonry_class: MasonryClass,
    pub accidental: bool,
}

impl AnnexParams {
    pub fn gamma_m(self) -> f64 {
        match self.annex {
            AnnexChoice::En => self.masonry_class.gamma_m_en(),
            AnnexChoice::De if self.accidental => 1.3,
            AnnexChoice::De => 1.5,
        }
    }
}
// #endregion 🔖️Annex

// #region 🔖️Part1_1
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
        CheckResult::from_utilization(ClauseId::new("EN 1996-1-1", "§6.2", "6.2"), Quantity::new(crate::document::QuantityKind::Moment, m_ed * 1_000_000.0), Quantity::new(crate::document::QuantityKind::Moment, m_rd * 1_000_000.0), "masonry flexure ULS", annex)
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
// #endregion 🔖️Part1_1

// #region 🔖️Part1_2
pub mod part_1_2 {
    use super::*;

    /// 🔥️ Minimum fire wall thickness [mm] per EN 1996-1-2 Table 5.1 (simplified).
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
// #endregion 🔖️Part1_2

// #region 🔖️Part2
/// 🧱️ EN 1996-2 selection of materials & execution: exposure-class durability admissibility and bed-joint execution checks. `ExposureClass`/`MortarClass` live in `crate::artifacts::en1996::part_2` (En1996Snapshot field types); this submodule holds only the compute functions.
pub mod part_2 {
    use super::*;
    use crate::artifacts::en1996::part_2::{ExposureClass, MortarClass};

    /// 📊️ Minimum admissible mortar strength [MPa] for a (unit, exposure) pair; ∞ marks an inadmissible combination.
    fn required_mortar_strength_mpa(exposure: ExposureClass, unit: MasonryUnit) -> f64 {
        match (exposure, unit) {
            (ExposureClass::Mx1, _) => 1.0,
            (ExposureClass::Mx2, MasonryUnit::Aac) => 2.5,
            (ExposureClass::Mx2, MasonryUnit::Clay) | (ExposureClass::Mx2, MasonryUnit::CalciumSilicate) => 5.0,
            (ExposureClass::Mx3, MasonryUnit::Clay) | (ExposureClass::Mx3, MasonryUnit::CalciumSilicate) => 10.0,
            (ExposureClass::Mx3, MasonryUnit::Aac) => f64::INFINITY,
            (ExposureClass::Mx4, MasonryUnit::Clay) => 20.0,
            (ExposureClass::Mx4, _) => f64::INFINITY,
            (ExposureClass::Mx5, MasonryUnit::Clay) => 20.0,
            (ExposureClass::Mx5, _) => f64::INFINITY,
        }
    }

    pub fn is_combination_admissible(exposure: ExposureClass, unit: MasonryUnit, mortar: MortarClass) -> bool {
        mortar.compressive_strength_mpa() >= required_mortar_strength_mpa(exposure, unit)
    }

    pub fn check_exposure_mortar(exposure: ExposureClass, unit: MasonryUnit, mortar: MortarClass) -> CheckResult {
        let required = required_mortar_strength_mpa(exposure, unit);
        CheckResult::from_minimum(
            ClauseId::new("EN 1996-2", "Annex B", "B.1"),
            Quantity::stress_mpa(mortar.compressive_strength_mpa()),
            Quantity::stress_mpa(required),
            format!("{} unit / {:?} mortar in exposure {:?}", unit.label(), mortar, exposure),
            AnnexChoice::En,
        )
    }

    /// 📏️ General-purpose mortar bed-joint thickness must fall within 6–15mm per EN 1996-2 §8.
    pub fn check_bed_joint_thickness(thickness_mm: f64) -> CheckResult {
        let clause = ClauseId::new("EN 1996-2", "§8", "8.1");
        let computed = Quantity::length_m(thickness_mm / 1000.0);
        let limit = Quantity::length_m(0.015);
        let within_range = (6.0..=15.0).contains(&thickness_mm);
        let utilization = thickness_mm / 15.0;
        if within_range {
            CheckResult::pass(clause, computed, limit, utilization, "bed-joint thickness within 6-15mm general-purpose mortar range", AnnexChoice::En)
        } else {
            CheckResult::fail(clause, computed, limit, utilization, "bed-joint thickness outside 6-15mm general-purpose mortar range", AnnexChoice::En)
        }
    }
}
// #endregion 🔖️Part2

// #region 🔖️Part3
/// 📐️ EN 1996-3 simplified Φ_s reduction-factor method for slender plain masonry walls (§4.2); prior retaining-wall/earth-pressure content was deleted as out-of-scope for EN 1996-3 — earth pressure and retaining-wall stability belong to EN 1997, and there is no clean EN 1996-1-1 "basement wall" analogue for it.
pub mod part_3 {
    use super::*;

    /// 📉️ Simplified capacity-reduction factor Φ_s per EN 1996-3 §4.2 (valid while ≥ 0).
    pub fn phi_s(h_ef_mm: f64, t_ef_mm: f64) -> f64 {
        let ratio = h_ef_mm / t_ef_mm;
        (0.85 - 0.0011 * ratio * ratio).max(0.0)
    }

    pub fn n_rd_kn(phi_s: f64, f_d_mpa: f64, area_mm2: f64) -> f64 {
        phi_s * f_d_mpa * area_mm2 / 1000.0
    }

    /// 🚧️ The simplified method only applies up to 3 storeys and a slenderness ratio of 27 (EN 1996-3 §1.1 scope).
    pub fn is_applicable(storeys: u32, h_ef_mm: f64, t_ef_mm: f64) -> bool {
        storeys <= 3 && h_ef_mm / t_ef_mm <= 27.0
    }

    #[allow(clippy::too_many_arguments, reason = "one argument per parameter the published clause formula itself names; bundling them into a struct would break the 1:1 reading against the standard")]
    pub fn check_simplified_compression(n_ed_kn: f64, phi_s: f64, f_d_mpa: f64, area_mm2: f64, storeys: u32, h_ef_mm: f64, t_ef_mm: f64, annex: AnnexChoice) -> CheckResult {
        let clause = ClauseId::new("EN 1996-3", "§4.2", "4.2");
        if !is_applicable(storeys, h_ef_mm, t_ef_mm) {
            return CheckResult {
                clause,
                status: CheckStatus::NotApplicable,
                computed: Quantity::force_kn(n_ed_kn),
                limit: Quantity::force_kn(0.0),
                utilization: 0.0,
                message: "simplified method not applicable: exceeds storey count or slenderness limit".into(),
                annex,
            };
        }
        let n_rd = n_rd_kn(phi_s, f_d_mpa, area_mm2);
        CheckResult::from_utilization(clause, Quantity::force_kn(n_ed_kn), Quantity::force_kn(n_rd), "simplified compression method N_Rd", annex)
    }
}
// #endregion 🔖️Part3

/// 📋️ Masonry wall under vertical load.
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

/// ⚖️ Derive the resolved γ_M annex parameters from a document's annex/class/situation inputs. Moved
/// here (from an inherent `En1996Snapshot::annex_params()` method in the pre-split monolith) because it
/// constructs the compute-layer `AnnexParams`, which cannot be an inherent impl on the foreign
/// `crate::artifacts::en1996::En1996Snapshot` type across the crate boundary (Rust's orphan rule).
pub fn annex_params(document: &En1996Snapshot) -> AnnexParams {
    AnnexParams { annex: document.annex, masonry_class: document.masonry_class, accidental: document.design_situation == DesignSituation::Accidental }
}

/// 📋️ Full EN 1996 check across flexure, compression, shear, sliding (part 1-1), fire wall (part 1-2), exposure/bed-joint (part 2), and the simplified method (part 3).
pub fn check_full_masonry(document: &En1996Snapshot) -> CheckReport {
    let g_m = annex_params(document).gamma_m();
    let f_d = part_1_1::design_strength_mpa(document.f_k_mpa, g_m);
    let f_vd = part_1_1::shear_design_strength_mpa(document.f_vk_mpa, g_m);
    let sigma = document.n_ed_kn * 1000.0 / document.area_mm2;
    let m_rd_flex = part_1_1::flexural_resistance_knm(document.z_mm3, f_d);
    let v_rd = part_1_1::shear_resistance_kn(document.shear_area_mm2, f_vd);
    let h_rd = part_1_1::sliding_resistance_kn(document.mu, document.n_ed_kn, f_vd, document.shear_area_mm2);
    let unit = parse_masonry_unit(&document.unit);
    let mut report = CheckReport::default();
    report.push(part_1_1::check_flexure(document.m_ed_knm, m_rd_flex, document.annex));
    report.push(part_1_1::check_compression(sigma, f_d, document.annex));
    report.push(part_1_1::check_shear(document.v_ed_kn, v_rd, document.annex));
    report.push(part_1_1::check_sliding(document.h_ed_kn, h_rd, document.annex));
    let required_fire = part_1_2::required_wall_thickness_mm(document.fire_resistance_min, unit);
    report.push(part_1_2::check_fire_wall(document.wall_thickness_mm, required_fire));
    report.push(part_2::check_exposure_mortar(document.exposure, unit, document.mortar));
    report.push(part_2::check_bed_joint_thickness(document.bed_joint_thickness_mm));
    let phi_s = part_3::phi_s(document.h_ef_mm, document.t_ef_mm);
    report.push(part_3::check_simplified_compression(document.n_ed_kn, phi_s, f_d, document.area_mm2, document.storeys, document.h_ef_mm, document.t_ef_mm, document.annex));
    report
}

/// 🧮️ Headless per-document evaluation — the `NormFamily::evaluate` body for `En1996Family` (defined
/// in the sibling `op` crate, which depends on this `engine` crate to call it).
pub fn evaluate(document: &En1996Snapshot) -> CheckReport {
    check_full_masonry(document)
}

// #region 🔖️Session
/// 🧩️ EN 1996's `NormFamily` binding — ties this artifact's `En1996Snapshot` to the `evaluate` above for the
/// headless `NormHost` session every norm app drives.
pub struct En1996Family;

impl crate::document::NormFamily for En1996Family {
    type Document = crate::artifacts::en1996::En1996Snapshot;
    type Mutation = crate::artifacts::en1996::mutations::En1996Mutation;

    fn family_id() -> crate::document::NormFamilyId {
        crate::document::NormFamilyId::En1996
    }

    fn evaluate(document: &Self::Document) -> crate::document::CheckReport {
        evaluate(document)
    }
}


//#region 🔖️ArtifactEngine
/// ⚙️ UI-independent En1996 artifact engine — owns the full artifact; `snapshot()` is persisted only.
pub struct En1996Engine {
    artifact: crate::artifacts::en1996::schema::En1996Artifact,
    snapshot: crate::artifacts::en1996::En1996Snapshot,
}

impl En1996Engine {
    pub fn new(snapshot: crate::artifacts::en1996::En1996Snapshot) -> Self {
        let artifact = crate::artifacts::en1996::schema::En1996Artifact::from_snapshot(snapshot.clone());
        Self { artifact, snapshot }
    }

    pub fn into_snapshot(self) -> crate::artifacts::en1996::En1996Snapshot {
        self.snapshot
    }
}
//#endregion 🔖️ArtifactEngine

pub type Host = crate::document::NormHost<En1996Family>;
// #endregion 🔖️Session

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn masonry_wall_sigma_vs_fd() {
        let sigma = 200.0 * 1000.0 / 500_000.0;
        let f_d = part_1_1::design_strength_mpa(5.0, na_de::gamma_m());
        assert!((sigma - 0.4_f64).abs() < 1e-9);
        assert!((f_d - (5.0 / 1.5)).abs() < 1e-9);
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
    fn fire_wall_r60_clay() {
        let required = part_1_2::required_wall_thickness_mm(60, MasonryUnit::Clay);
        assert!((required - 90.0).abs() < 0.1);
    }

    #[test]
    fn exposure_mortar_mx1_clay_m1_admissible() {
        let result = part_2::check_exposure_mortar(crate::artifacts::en1996::part_2::ExposureClass::Mx1, MasonryUnit::Clay, crate::artifacts::en1996::part_2::MortarClass::M1);
        assert_eq!(result.status, CheckStatus::Pass);
    }

    #[test]
    fn exposure_mortar_mx4_aac_inadmissible() {
        let result = part_2::check_exposure_mortar(crate::artifacts::en1996::part_2::ExposureClass::Mx4, MasonryUnit::Aac, crate::artifacts::en1996::part_2::MortarClass::M20);
        assert_eq!(result.status, CheckStatus::Fail);
    }

    #[test]
    fn bed_joint_thickness_range() {
        let ok = part_2::check_bed_joint_thickness(12.0);
        assert_eq!(ok.status, CheckStatus::Pass);
        let too_thin = part_2::check_bed_joint_thickness(3.0);
        assert_eq!(too_thin.status, CheckStatus::Fail);
    }

    #[test]
    fn simplified_method_worked_example() {
        let phi_s = part_3::phi_s(3600.0, 240.0);
        assert!((phi_s - 0.6025).abs() < 1e-9);
        let gamma_m = AnnexParams { annex: AnnexChoice::De, masonry_class: MasonryClass::Class3, accidental: false }.gamma_m();
        assert!((gamma_m - 1.5).abs() < 1e-9);
        let f_d = part_1_1::design_strength_mpa(5.0, gamma_m);
        assert!((f_d - 10.0 / 3.0).abs() < 1e-9);
        let n_rd = part_3::n_rd_kn(phi_s, f_d, 240_000.0);
        assert!((n_rd - 482.0).abs() < 0.5);
    }

    #[test]
    fn simplified_method_applicability_guard() {
        assert!(part_3::is_applicable(2, 2500.0, 240.0));
        assert!(!part_3::is_applicable(4, 2500.0, 240.0));
        assert!(!part_3::is_applicable(2, 8000.0, 240.0));
        let result = part_3::check_simplified_compression(100.0, 0.6, 3.0, 100_000.0, 4, 2500.0, 240.0, AnnexChoice::De);
        assert_eq!(result.status, CheckStatus::NotApplicable);
    }

    #[test]
    fn gamma_m_diverges_en_class2_vs_de_flat() {
        let en = AnnexParams { annex: AnnexChoice::En, masonry_class: MasonryClass::Class2, accidental: false };
        let de = AnnexParams { annex: AnnexChoice::De, masonry_class: MasonryClass::Class2, accidental: false };
        assert!((en.gamma_m() - 1.7).abs() < 1e-9);
        assert!((de.gamma_m() - 1.5).abs() < 1e-9);
        let phi_s = part_3::phi_s(3600.0, 240.0);
        let area_mm2 = 240_000.0;
        let n_rd_en = part_3::n_rd_kn(phi_s, part_1_1::design_strength_mpa(5.0, en.gamma_m()), area_mm2);
        let n_rd_de = part_3::n_rd_kn(phi_s, part_1_1::design_strength_mpa(5.0, de.gamma_m()), area_mm2);
        assert!(n_rd_de > n_rd_en);
        assert!((n_rd_de - n_rd_en).abs() > 1.0);
    }

    #[test]
    fn full_masonry_worked_example() {
        let report = check_full_masonry(&En1996Snapshot::default());
        assert_eq!(report.checks.len(), 8);
    }

    #[test]
    fn evaluate_runs_all_parts() {
        let report = evaluate(&En1996Snapshot::default());
        assert_eq!(report.checks.len(), 8);
    }
}
//#endregion 🧪️Tests


/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    register_artifact_schema();
    dsl::register_language(dsl::LanguageSpec {
        id: "en1996.document",
        extension: Some("en1996"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::en1995::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::en1995::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::en1995::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::en1995::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("en1996.document"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "en1996.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::en1995::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::en1995::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::en1995::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::en1995::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("en1996.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "en1996.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::en1995::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::en1995::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("en1996.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "en1996.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::en1995::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::en1995::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("en1996.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "en1996.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::en1995::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::en1995::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("en1996.spr"),
    });
}

//#region 🔖️Register
/// 🧷️ Registers this artifact's pilot languages/schema descriptor (via `register_pilot_languages`,
/// which already registers the schema) and its inference descriptor in one call — the fan-out
/// target for the plugin root's `.setup()` hook (folded in from the deleted `🔧️setup` facet, APA).
pub fn register() {
    register_pilot_languages();
    register_artifact_inferences();
}
//#endregion 🔖️Register

//#region 🔖️SchemaRegistry
use std::sync::{Mutex, OnceLock};

/// 📌️ Registers the twenty handcrafted schema leaves for `s.norm.en1996`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::en1996::schema::en1996_artifact_schema_descriptor());
}

/// 💡️ Registers `s.norm.en1996.inference`'s facet leaves into the OS-wide inference catalog —
/// sibling to `register_artifact_schema()` above (separate registry, ticket
/// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
pub fn register_artifact_inferences() {
    ::schema::register_artifact_inference_descriptor(crate::artifacts::en1996::standards::v1::subsets::any::schema::inferences::en1996_artifact_inference_descriptor());
}
//#endregion 🔖️SchemaRegistry

//#region 🔖️IoFacet
pub fn register_io() {
    crate::artifacts::en1996::io_registry::register();
}
//#endregion 🔖️IoFacet
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, composer_entry_of};
    use crate::artifacts::en1996::standards::v1::subsets::any::schema::En1996Composer as En1996AnyComposer;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![
            composer_entry_of::<En1996AnyComposer>(),
        ]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
