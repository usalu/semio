//! 🧱️ EN 1996 artifact schema — every field with its state class.

use crate::artifacts::en1996::En1996Snapshot;
use crate::artifacts::en1996::MasonryClass;
use crate::document::{AnnexChoice, CheckReport, CheckResult, CheckStatus, ClauseId, Quantity};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full EN 1996 artifact state (persisted document + shared UI).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1996")]
pub struct En1996Artifact {
    #[state(artifact)]
    pub m_ed_knm: f64,
    #[state(artifact)]
    pub n_ed_kn: f64,
    #[state(artifact)]
    pub v_ed_kn: f64,
    #[state(artifact)]
    pub h_ed_kn: f64,
    #[state(artifact)]
    pub z_mm3: f64,
    #[state(artifact)]
    pub area_mm2: f64,
    #[state(artifact)]
    pub shear_area_mm2: f64,
    #[state(artifact)]
    pub f_k_mpa: f64,
    #[state(artifact)]
    pub f_vk_mpa: f64,
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub masonry_class: MasonryClass,
    #[state(artifact)]
    pub design_situation: crate::document::DesignSituation,
    #[state(artifact)]
    pub mu: f64,
    #[state(artifact)]
    pub wall_thickness_mm: f64,
    #[state(artifact)]
    pub fire_resistance_min: u32,
    #[state(artifact)]
    pub unit: String,
    #[state(artifact)]
    pub exposure: crate::artifacts::en1996::part_2::ExposureClass,
    #[state(artifact)]
    pub mortar: crate::artifacts::en1996::part_2::MortarClass,
    #[state(artifact)]
    pub bed_joint_thickness_mm: f64,
    #[state(artifact)]
    pub storeys: u32,
    #[state(artifact)]
    pub h_ef_mm: f64,
    #[state(artifact)]
    pub t_ef_mm: f64,
    #[state(presence)]
    pub selected_check_index: Option<u32>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for En1996Artifact {
    fn default() -> Self {
        Self::from_snapshot(En1996Snapshot::default())
    }
}

impl From<En1996Snapshot> for En1996Artifact {
    fn from(snapshot: En1996Snapshot) -> Self {
        Self::from_snapshot(snapshot)
    }
}

impl En1996Artifact {
    pub fn to_snapshot(&self) -> En1996Snapshot {
        En1996Snapshot {
            m_ed_knm: self.m_ed_knm.clone(),
            n_ed_kn: self.n_ed_kn.clone(),
            v_ed_kn: self.v_ed_kn.clone(),
            h_ed_kn: self.h_ed_kn.clone(),
            z_mm3: self.z_mm3.clone(),
            area_mm2: self.area_mm2.clone(),
            shear_area_mm2: self.shear_area_mm2.clone(),
            f_k_mpa: self.f_k_mpa.clone(),
            f_vk_mpa: self.f_vk_mpa.clone(),
            annex: self.annex.clone(),
            masonry_class: self.masonry_class.clone(),
            design_situation: self.design_situation.clone(),
            mu: self.mu.clone(),
            wall_thickness_mm: self.wall_thickness_mm.clone(),
            fire_resistance_min: self.fire_resistance_min.clone(),
            unit: self.unit.clone(),
            exposure: self.exposure.clone(),
            mortar: self.mortar.clone(),
            bed_joint_thickness_mm: self.bed_joint_thickness_mm.clone(),
            storeys: self.storeys.clone(),
            h_ef_mm: self.h_ef_mm.clone(),
            t_ef_mm: self.t_ef_mm.clone(),
        }
    }

    pub fn from_snapshot(snapshot: En1996Snapshot) -> Self {
        Self {
            m_ed_knm: snapshot.m_ed_knm,
            n_ed_kn: snapshot.n_ed_kn,
            v_ed_kn: snapshot.v_ed_kn,
            h_ed_kn: snapshot.h_ed_kn,
            z_mm3: snapshot.z_mm3,
            area_mm2: snapshot.area_mm2,
            shear_area_mm2: snapshot.shear_area_mm2,
            f_k_mpa: snapshot.f_k_mpa,
            f_vk_mpa: snapshot.f_vk_mpa,
            annex: snapshot.annex,
            masonry_class: snapshot.masonry_class,
            design_situation: snapshot.design_situation,
            mu: snapshot.mu,
            wall_thickness_mm: snapshot.wall_thickness_mm,
            fire_resistance_min: snapshot.fire_resistance_min,
            unit: snapshot.unit,
            exposure: snapshot.exposure,
            mortar: snapshot.mortar,
            bed_joint_thickness_mm: snapshot.bed_joint_thickness_mm,
            storeys: snapshot.storeys,
            h_ef_mm: snapshot.h_ef_mm,
            t_ef_mm: snapshot.t_ef_mm,
            selected_check_index: None,
        }
    }

    pub fn set_snapshot(&mut self, snapshot: En1996Snapshot) {
        self.m_ed_knm = snapshot.m_ed_knm;
        self.n_ed_kn = snapshot.n_ed_kn;
        self.v_ed_kn = snapshot.v_ed_kn;
        self.h_ed_kn = snapshot.h_ed_kn;
        self.z_mm3 = snapshot.z_mm3;
        self.area_mm2 = snapshot.area_mm2;
        self.shear_area_mm2 = snapshot.shear_area_mm2;
        self.f_k_mpa = snapshot.f_k_mpa;
        self.f_vk_mpa = snapshot.f_vk_mpa;
        self.annex = snapshot.annex;
        self.masonry_class = snapshot.masonry_class;
        self.design_situation = snapshot.design_situation;
        self.mu = snapshot.mu;
        self.wall_thickness_mm = snapshot.wall_thickness_mm;
        self.fire_resistance_min = snapshot.fire_resistance_min;
        self.unit = snapshot.unit;
        self.exposure = snapshot.exposure;
        self.mortar = snapshot.mortar;
        self.bed_joint_thickness_mm = snapshot.bed_joint_thickness_mm;
        self.storeys = snapshot.storeys;
        self.h_ef_mm = snapshot.h_ef_mm;
        self.t_ef_mm = snapshot.t_ef_mm;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
pub fn en1996_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.norm.en1996",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::en1996::{En1996Diff, En1996Mutation, En1996Snapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct En1996BuilderConstruction {
        snapshot: En1996Snapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for En1996BuilderConstruction {
        type Snapshot = En1996Snapshot;
        type Mutation = En1996Mutation;
        type Diff = En1996Diff;
        fn empty() -> Self {
            Self { snapshot: En1996Snapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<En1996Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<En1996Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error(
                    "mutation.apply",
                    dsl::TextSpan::at(1, 1),
                    error.to_string(),
                )),
            }
            (self, outcome)
        }
        fn absorb(
            mut self,
            diff: Self::Diff,
        ) -> protocol::MutationApplyResult<Self> {
            let snapshot = <En1996Diff as protocol::MutationDiff<En1996Snapshot>>::apply(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::en1996::En1996Snapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct En1996Parts {
        pub snapshot: Option<En1996Snapshot>,
    }

    pub struct En1996AnalyzerAnalysis;

    impl ArtifactAnalysis for En1996AnalyzerAnalysis {
        type Parts = En1996Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.en1996", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = En1996Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <En1996Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <En1996Snapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec En1996BuilderFacets {
        construction: En1996BuilderConstruction,
        analysis: En1996AnalyzerAnalysis,
        composition: super::super::io::derived_composition::En1996ComposerComposition,
    }
    builder: En1996Builder,
    analyzer: En1996Analyzer,
    composer: En1996Composer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️ComplianceHelpers
/// 📐️ Pure EN 1996 compliance helpers (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// relocated verbatim from the deleted `⚙️engine`. `na_de`, `MasonryUnit`, `AnnexParams`,
/// `part_1_1`/`part_1_2`/`part_3` and `check_masonry_wall` are pure function libraries; the
/// snapshot-level composition (`evaluate`, `check_full_masonry`, `annex_params`) lives in
/// `💡️inferences`. `na_de` re-exports `crate::artifacts::en1990`'s relocated `NaDe`.
pub mod na_de {
    pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::na_de::NaDe;

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
        CheckResult::from_utilization(
            ClauseId::new("EN 1996-1-1", "§6.2", "6.2"),
            Quantity::new(crate::document::QuantityKind::Moment, m_ed * 1_000_000.0),
            Quantity::new(crate::document::QuantityKind::Moment, m_rd * 1_000_000.0),
            "masonry flexure ULS",
            annex,
        )
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

//#endregion 🔖️ComplianceHelpers

//#region 🧪️ComplianceHelpersTests
#[cfg(test)]
mod compliance_helpers_tests {
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
}
//#endregion 🧪️ComplianceHelpersTests
