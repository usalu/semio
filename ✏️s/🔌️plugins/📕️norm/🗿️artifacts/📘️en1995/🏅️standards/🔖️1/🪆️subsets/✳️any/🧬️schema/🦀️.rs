//! 🪵️ EN 1995 artifact schema — every field with its state class.

use crate::artifacts::en1995::En1995Snapshot;
use crate::document::AnnexChoice;
use schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ Full EN 1995 artifact state (persisted document + shared UI).
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1995")]
pub struct En1995Artifact {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub m_ed_knm: f64,
    #[state(artifact)]
    pub n_ed_kn: f64,
    #[state(artifact)]
    pub v_ed_kn: f64,
    #[state(artifact)]
    pub w_mm3: f64,
    #[state(artifact)]
    pub a_mm2: f64,
    #[state(artifact)]
    pub b_mm: f64,
    #[state(artifact)]
    pub h_mm: f64,
    #[state(artifact)]
    pub f_m_k: f64,
    #[state(artifact)]
    pub f_c_0_k: f64,
    #[state(artifact)]
    pub service_class: String,
    #[state(artifact)]
    pub load_duration: String,
    #[state(artifact)]
    pub m_crit_knm: f64,
    #[state(artifact)]
    pub f_ed_kn: f64,
    #[state(artifact)]
    pub a_ef_mm2: f64,
    #[state(artifact)]
    pub f_v_k: f64,
    #[state(artifact)]
    pub fire_duration_min: f64,
    #[state(artifact)]
    pub section_depth_mm: f64,
    #[state(artifact)]
    pub a_vert_m_s2: f64,
    #[state(artifact)]
    pub n_cycles_bridge: f64,
    #[state(presence)]
    pub selected_check_index: Option<u32>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for En1995Artifact {
    fn default() -> Self {
        Self::from_snapshot(En1995Snapshot::default())
    }
}

impl From<En1995Snapshot> for En1995Artifact {
    fn from(snapshot: En1995Snapshot) -> Self {
        Self::from_snapshot(snapshot)
    }
}

impl En1995Artifact {
    pub fn to_snapshot(&self) -> En1995Snapshot {
        En1995Snapshot {
            annex: self.annex.clone(),
            m_ed_knm: self.m_ed_knm.clone(),
            n_ed_kn: self.n_ed_kn.clone(),
            v_ed_kn: self.v_ed_kn.clone(),
            w_mm3: self.w_mm3.clone(),
            a_mm2: self.a_mm2.clone(),
            b_mm: self.b_mm.clone(),
            h_mm: self.h_mm.clone(),
            f_m_k: self.f_m_k.clone(),
            f_c_0_k: self.f_c_0_k.clone(),
            service_class: self.service_class.clone(),
            load_duration: self.load_duration.clone(),
            m_crit_knm: self.m_crit_knm.clone(),
            f_ed_kn: self.f_ed_kn.clone(),
            a_ef_mm2: self.a_ef_mm2.clone(),
            f_v_k: self.f_v_k.clone(),
            fire_duration_min: self.fire_duration_min.clone(),
            section_depth_mm: self.section_depth_mm.clone(),
            a_vert_m_s2: self.a_vert_m_s2.clone(),
            n_cycles_bridge: self.n_cycles_bridge.clone(),
        }
    }

    pub fn from_snapshot(snapshot: En1995Snapshot) -> Self {
        Self {
            annex: snapshot.annex,
            m_ed_knm: snapshot.m_ed_knm,
            n_ed_kn: snapshot.n_ed_kn,
            v_ed_kn: snapshot.v_ed_kn,
            w_mm3: snapshot.w_mm3,
            a_mm2: snapshot.a_mm2,
            b_mm: snapshot.b_mm,
            h_mm: snapshot.h_mm,
            f_m_k: snapshot.f_m_k,
            f_c_0_k: snapshot.f_c_0_k,
            service_class: snapshot.service_class,
            load_duration: snapshot.load_duration,
            m_crit_knm: snapshot.m_crit_knm,
            f_ed_kn: snapshot.f_ed_kn,
            a_ef_mm2: snapshot.a_ef_mm2,
            f_v_k: snapshot.f_v_k,
            fire_duration_min: snapshot.fire_duration_min,
            section_depth_mm: snapshot.section_depth_mm,
            a_vert_m_s2: snapshot.a_vert_m_s2,
            n_cycles_bridge: snapshot.n_cycles_bridge,
            selected_check_index: None,
        }
    }

    pub fn set_snapshot(&mut self, snapshot: En1995Snapshot) {
        self.annex = snapshot.annex;
        self.m_ed_knm = snapshot.m_ed_knm;
        self.n_ed_kn = snapshot.n_ed_kn;
        self.v_ed_kn = snapshot.v_ed_kn;
        self.w_mm3 = snapshot.w_mm3;
        self.a_mm2 = snapshot.a_mm2;
        self.b_mm = snapshot.b_mm;
        self.h_mm = snapshot.h_mm;
        self.f_m_k = snapshot.f_m_k;
        self.f_c_0_k = snapshot.f_c_0_k;
        self.service_class = snapshot.service_class;
        self.load_duration = snapshot.load_duration;
        self.m_crit_knm = snapshot.m_crit_knm;
        self.f_ed_kn = snapshot.f_ed_kn;
        self.a_ef_mm2 = snapshot.a_ef_mm2;
        self.f_v_k = snapshot.f_v_k;
        self.fire_duration_min = snapshot.fire_duration_min;
        self.section_depth_mm = snapshot.section_depth_mm;
        self.a_vert_m_s2 = snapshot.a_vert_m_s2;
        self.n_cycles_bridge = snapshot.n_cycles_bridge;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
pub fn en1995_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.norm.en1995",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::en1995::{En1995Diff, En1995Mutation, En1995Snapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct En1995BuilderConstruction {
        snapshot: En1995Snapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for En1995BuilderConstruction {
        type Snapshot = En1995Snapshot;
        type Mutation = En1995Mutation;
        type Diff = En1995Diff;
        fn empty() -> Self {
            Self { snapshot: En1995Snapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<En1995Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<En1995Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <En1995Diff as protocol::MutationDiff<En1995Snapshot>>::apply(&diff, &self.snapshot)?;
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
    use crate::artifacts::en1995::En1995Snapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct En1995Parts {
        pub snapshot: Option<En1995Snapshot>,
    }

    pub struct En1995AnalyzerAnalysis;

    impl ArtifactAnalysis for En1995AnalyzerAnalysis {
        type Parts = En1995Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.en1995", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = En1995Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <En1995Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <En1995Snapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec En1995BuilderFacets {
        construction: En1995BuilderConstruction,
        analysis: En1995AnalyzerAnalysis,
        composition: super::super::io::derived_composition::En1995ComposerComposition,
    }
    builder: En1995Builder,
    analyzer: En1995Analyzer,
    composer: En1995Composer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️ComplianceHelpers
/// 📐️ Pure EN 1995 compliance helpers (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// relocated verbatim from the deleted `⚙️engine`. `na_de`, `AnnexParams`, the `Kmod` region,
/// `part_1_1`/`part_1_2`/`part_2` and `check_glulam_beam` are pure function libraries; the
/// snapshot-level composition (`evaluate`, `check_full_timber`) lives in `💡️inferences`.
/// `na_de` re-exports `crate::artifacts::en1990`'s relocated `NaDe`.
use crate::document::{CheckReport, CheckResult, ClauseId, LoadDuration, Quantity};

pub mod na_de {
    pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::na_de::NaDe;
}

// #region 🔖️AnnexParams
/// 🪵️ Timber material category for γ_M selection per EN 1995-1-1 Table 2.3.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimberMaterial {
    Solid,
    Glulam,
    Connection,
}

/// 🇪️🇺️ National-annex NDPs for EN 1995: γ_M per material (unchanged by the German NA) and the shear-effective-width factor k_cr, which genuinely diverges between EN and DIN EN 1995-1-1/NA.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AnnexParams {
    pub choice: AnnexChoice,
}

impl AnnexParams {
    pub fn en() -> Self {
        Self { choice: AnnexChoice::En }
    }

    pub fn de() -> Self {
        Self { choice: AnnexChoice::De }
    }

    pub fn for_annex(annex: AnnexChoice) -> Self {
        match annex {
            AnnexChoice::En => Self::en(),
            AnnexChoice::De => Self::de(),
        }
    }

    /// 📖️ γ_M per material per EN 1995-1-1 Table 2.3 — EN-recommended, not amended by DIN EN 1995-1-1/NA.
    pub fn gamma_m(&self, material: TimberMaterial) -> f64 {
        match material {
            TimberMaterial::Solid => 1.3,
            TimberMaterial::Glulam => 1.25,
            TimberMaterial::Connection => 1.3,
        }
    }

    /// 🇩️🇪️ k_cr shear-effective-width factor: EN 1995-1-1 §6.1.7(2) recommends the constant 0.67; DIN EN 1995-1-1/NA instead specifies min(1.0, 2.5/f_v,k) with f_v,k in MPa.
    pub fn k_cr(&self, f_v_k_mpa: f64) -> f64 {
        match self.choice {
            AnnexChoice::En => 0.67,
            AnnexChoice::De => (2.5 / f_v_k_mpa).min(1.0),
        }
    }
}
// #endregion 🔖️AnnexParams

/// 🌡️ Service class per EN 1995-1-1 Table 3.1.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ServiceClass {
    Sc1,
    Sc2,
    Sc3,
}

// #region 🔖️Kmod
/// 📊️ k_mod per EN 1995-1-1 Table 3.1 (service class × load duration).
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

/// 📊️ k_def per EN 1995-1-1 Table 3.2.
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

/// 📐️ Lateral torsional buckling factor k_crit per EN 1995-1-1 Eq. 6.25.
pub fn k_crit(lambda_rel_m: f64) -> f64 {
    if lambda_rel_m <= 0.75 {
        1.0
    } else if lambda_rel_m <= 1.4 {
        1.0 / (lambda_rel_m + (lambda_rel_m * lambda_rel_m - 0.75_f64.powi(2)).sqrt())
    } else {
        1.0 / (lambda_rel_m * lambda_rel_m)
    }
}

/// 📐️ Relative slenderness λ_rel,m for lateral torsional buckling.
pub fn lambda_rel_m(w_eff_mm3: f64, f_m_k_mpa: f64, m_crit_knm: f64) -> f64 {
    (w_eff_mm3 * f_m_k_mpa / 1_000_000.0 / m_crit_knm).sqrt()
}
// #endregion 🔖️Kmod

// #region 🔖️Part1_1
pub mod part_1_1 {
    use super::*;

    pub fn bending_resistance_knm(w_mm3: f64, f_m_k_mpa: f64, k_mod: f64, k_crit: f64, annex: AnnexChoice) -> f64 {
        let gamma_m = AnnexParams::for_annex(annex).gamma_m(TimberMaterial::Glulam);
        k_mod * k_crit * w_mm3 * f_m_k_mpa / gamma_m / 1_000_000.0
    }

    pub fn compression_resistance_kn(a_mm2: f64, f_c_0_k_mpa: f64, k_mod: f64, annex: AnnexChoice) -> f64 {
        let gamma_m = AnnexParams::for_annex(annex).gamma_m(TimberMaterial::Glulam);
        k_mod * a_mm2 * f_c_0_k_mpa / gamma_m / 1000.0
    }

    pub fn connection_bearing_resistance_kn(a_ef_mm2: f64, f_v_k_mpa: f64, k_mod: f64, annex: AnnexChoice) -> f64 {
        let gamma_m = AnnexParams::for_annex(annex).gamma_m(TimberMaterial::Connection);
        k_mod * a_ef_mm2 * f_v_k_mpa / gamma_m / 1000.0
    }

    /// 📐️ Shear stress τ [MPa] per EN 1995-1-1 §6.1.7 Eq. 6.13a, using the k_cr-reduced effective width b_ef = k_cr·b.
    pub fn shear_stress_mpa(v_ed_kn: f64, b_mm: f64, h_mm: f64, k_cr: f64) -> f64 {
        let b_ef_mm = k_cr * b_mm;
        1.5 * v_ed_kn * 1000.0 / (b_ef_mm * h_mm)
    }

    /// 📐️ Shear design resistance f_v,d [MPa] per EN 1995-1-1 §2.4.1, Eq. 2.14.
    pub fn shear_resistance_mpa(f_v_k_mpa: f64, k_mod: f64, annex: AnnexChoice) -> f64 {
        let gamma_m = AnnexParams::for_annex(annex).gamma_m(TimberMaterial::Solid);
        k_mod * f_v_k_mpa / gamma_m
    }

    pub fn check_bending(m_ed: f64, m_rd: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1995-1-1", "§6.1.6", "6.1"),
            Quantity::new(crate::document::QuantityKind::Moment, m_ed * 1_000_000.0),
            Quantity::new(crate::document::QuantityKind::Moment, m_rd * 1_000_000.0),
            "timber bending ULS",
            annex,
        )
    }

    pub fn check_compression(n_ed_kn: f64, n_rd_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1995-1-1", "§6.1.4", "6.1"), Quantity::force_kn(n_ed_kn), Quantity::force_kn(n_rd_kn), "timber compression ULS", annex)
    }

    pub fn check_connection_bearing(f_ed_kn: f64, f_rd_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1995-1-1", "§8.1.2", "8.1"), Quantity::force_kn(f_ed_kn), Quantity::force_kn(f_rd_kn), "timber connection bearing ULS", annex)
    }

    /// 📐️ Timber shear ULS check per EN 1995-1-1 §6.1.7 — utilization diverges between EN and DE via k_cr.
    pub fn check_shear(v_ed_kn: f64, b_mm: f64, h_mm: f64, f_v_k_mpa: f64, k_mod: f64, annex: AnnexChoice) -> CheckResult {
        let k_cr = AnnexParams::for_annex(annex).k_cr(f_v_k_mpa);
        let tau = shear_stress_mpa(v_ed_kn, b_mm, h_mm, k_cr);
        let f_v_d = shear_resistance_mpa(f_v_k_mpa, k_mod, annex);
        CheckResult::from_utilization(ClauseId::new("EN 1995-1-1", "§6.1.7", "6.1.7"), Quantity::stress_mpa(tau), Quantity::stress_mpa(f_v_d), "timber shear ULS (k_cr effective width)", annex)
    }
}
// #endregion 🔖️Part1_1

// #region 🔖️Part1_2
pub mod part_1_2 {
    use super::*;

    /// 🔥️ One-dimensional charring rate β₀ [mm/min] per EN 1995-1-2 §4.2.2.
    pub const CHARRING_RATE_MM_MIN: f64 = 0.65;

    pub fn charred_depth_mm(fire_duration_min: f64) -> f64 {
        CHARRING_RATE_MM_MIN * fire_duration_min
    }

    pub fn residual_section_mm(original_mm: f64, charred_depth_mm: f64) -> f64 {
        (original_mm - 2.0 * charred_depth_mm).max(0.0)
    }

    pub fn check_fire(charred_depth_mm: f64, remaining_mm: f64) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1995-1-2", "§4", "4.2"), Quantity::length_m(remaining_mm / 1000.0), Quantity::length_m(charred_depth_mm / 1000.0), "timber fire residual section", AnnexChoice::De)
    }
}
// #endregion 🔖️Part1_2

// #region 🔖️Part2
pub mod part_2 {
    use super::{k_mod, part_1_1, AnnexChoice, CheckResult, ClauseId, LoadDuration, Quantity, ServiceClass};

    /// 🌉️ Pedestrian comfort limit for vertical acceleration [m/s²] per EN 1995-2 §7 (serviceability, footbridges).
    pub const A_VERT_LIMIT_M_S2: f64 = 0.7;

    /// 🌉️ Reference cycle count below which no fatigue reduction applies, per EN 1995-2 Annex A style guidance.
    pub const FATIGUE_N0_CYCLES: f64 = 1.0e6;

    pub fn bridge_bending_resistance_knm(w_mm3: f64, f_m_k_mpa: f64, service: ServiceClass, duration: LoadDuration, k_crit: f64, annex: AnnexChoice) -> f64 {
        part_1_1::bending_resistance_knm(w_mm3, f_m_k_mpa, k_mod(service, duration), k_crit, annex)
    }

    pub fn check_bridge_timber(m_ed: f64, m_rd: f64) -> CheckResult {
        part_1_1::check_bending(m_ed, m_rd, AnnexChoice::En)
    }

    /// 🌉️ Pedestrian-induced vertical vibration serviceability check per EN 1995-2 §7.
    pub fn check_pedestrian_vibration(a_vert_m_s2: f64) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1995-2", "§7", "7.1"), Quantity::acceleration_m_s2(a_vert_m_s2), Quantity::acceleration_m_s2(A_VERT_LIMIT_M_S2), "pedestrian vertical vibration comfort", AnnexChoice::En)
    }

    /// 📐️ Simplified fatigue/damage reduction factor k_fat for repeated traffic loading, per EN 1995-2 Annex A style S-N degradation (reference N0 = 1e6 cycles, 10% reduction per decade, floored at 0.5).
    pub fn fatigue_reduction_factor(n_cycles: f64) -> f64 {
        if n_cycles <= FATIGUE_N0_CYCLES {
            1.0
        } else {
            (1.0 - 0.1 * (n_cycles / FATIGUE_N0_CYCLES).log10()).max(0.5)
        }
    }

    /// 🌉️ Bridge bending check under repeated traffic loading, with the base resistance reduced by k_fat per EN 1995-2 Annex A.
    pub fn check_bridge_fatigue(m_ed_knm: f64, m_rd_knm: f64, n_cycles: f64) -> CheckResult {
        let k_fat = fatigue_reduction_factor(n_cycles);
        let m_rd_fatigue_knm = m_rd_knm * k_fat;
        CheckResult::from_utilization(
            ClauseId::new("EN 1995-2", "Annex A", "A.1"),
            Quantity::new(crate::document::QuantityKind::Moment, m_ed_knm * 1_000_000.0),
            Quantity::new(crate::document::QuantityKind::Moment, m_rd_fatigue_knm * 1_000_000.0),
            "bridge timber fatigue-reduced bending",
            AnnexChoice::En,
        )
    }
}
// #endregion 🔖️Part2

/// 📋️ Glulam beam check with LTB, compression, and k_cr-dependent shear.
#[allow(clippy::too_many_arguments)]
pub fn check_glulam_beam(
    m_ed_knm: f64,
    n_ed_kn: f64,
    v_ed_kn: f64,
    w_mm3: f64,
    a_mm2: f64,
    b_mm: f64,
    h_mm: f64,
    f_m_k: f64,
    f_c_0_k: f64,
    f_v_k: f64,
    service: ServiceClass,
    duration: LoadDuration,
    m_crit_knm: f64,
    annex: AnnexChoice,
) -> CheckReport {
    let km = k_mod(service, duration);
    let lambda = lambda_rel_m(w_mm3, f_m_k, m_crit_knm);
    let kc = k_crit(lambda);
    let m_rd = part_1_1::bending_resistance_knm(w_mm3, f_m_k, km, kc, annex);
    let n_rd = part_1_1::compression_resistance_kn(a_mm2, f_c_0_k, km, annex);
    let mut report = CheckReport::default();
    report.push(part_1_1::check_bending(m_ed_knm, m_rd, annex));
    report.push(part_1_1::check_compression(n_ed_kn, n_rd, annex));
    report.push(part_1_1::check_shear(v_ed_kn, b_mm, h_mm, f_v_k, km, annex));
    report
}

//#endregion 🔖️ComplianceHelpers

//#region 🧪️ComplianceHelpersTests
#[cfg(test)]
mod compliance_helpers_tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    fn k_mod_sc1_permanent() {
        assert!((k_mod(ServiceClass::Sc1, LoadDuration::Permanent) - 0.6).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    fn glulam_ltb_reduces_bending_resistance() {
        let w = 1_000_000.0;
        let f_m_k = 24.0;
        let km = k_mod(ServiceClass::Sc1, LoadDuration::Medium);
        let m_crit = 30.0;
        let lambda = lambda_rel_m(w, f_m_k, m_crit);
        let kc = k_crit(lambda);
        assert!(kc < 1.0);
        let m_rd = part_1_1::bending_resistance_knm(w, f_m_k, km, kc, AnnexChoice::De);
        let m_rd_full = part_1_1::bending_resistance_knm(w, f_m_k, km, 1.0, AnnexChoice::De);
        assert!(m_rd < m_rd_full);
    }

    #[semio_framework_async_macros::async_test]
    fn glulam_beam_e2e() {
        let report = check_glulam_beam(25.0, 50.0, 15.0, 1_000_000.0, 20_000.0, 200.0, 300.0, 24.0, 21.0, 4.0, ServiceClass::Sc1, LoadDuration::Medium, 80.0, AnnexChoice::De);
        assert_eq!(report.checks.len(), 3);
    }

    #[semio_framework_async_macros::async_test]
    fn connection_bearing_resistance_worked() {
        let km = k_mod(ServiceClass::Sc1, LoadDuration::Medium);
        let f_rd = part_1_1::connection_bearing_resistance_kn(12_000.0, 2.5, km, AnnexChoice::De);
        assert!((f_rd - 18.46).abs() < 0.5);
    }

    #[semio_framework_async_macros::async_test]
    fn fire_char_depth_r30() {
        let charred = part_1_2::charred_depth_mm(30.0);
        assert!((charred - 19.5).abs() < 0.1);
        let remaining = part_1_2::residual_section_mm(300.0, charred);
        assert!((remaining - 261.0).abs() < 0.1);
    }

    #[semio_framework_async_macros::async_test]
    fn annex_params_gamma_m_per_material() {
        let en = AnnexParams::en();
        let de = AnnexParams::de();
        assert!((en.gamma_m(TimberMaterial::Solid) - 1.3).abs() < 1e-9);
        assert!((en.gamma_m(TimberMaterial::Glulam) - 1.25).abs() < 1e-9);
        assert!((en.gamma_m(TimberMaterial::Connection) - 1.3).abs() < 1e-9);
        assert!((en.gamma_m(TimberMaterial::Solid) - de.gamma_m(TimberMaterial::Solid)).abs() < 1e-9);
        assert!((en.gamma_m(TimberMaterial::Glulam) - de.gamma_m(TimberMaterial::Glulam)).abs() < 1e-9);
        assert!((en.gamma_m(TimberMaterial::Connection) - de.gamma_m(TimberMaterial::Connection)).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    fn k_cr_diverges_between_en_and_de_for_c24() {
        let f_v_k = 4.0;
        let k_cr_en = AnnexParams::en().k_cr(f_v_k);
        let k_cr_de = AnnexParams::de().k_cr(f_v_k);
        assert!((k_cr_en - 0.67).abs() < 1e-9);
        assert!((k_cr_de - 0.625).abs() < 1e-9);
        assert!((k_cr_en - k_cr_de).abs() > 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    fn shear_utilization_diverges_between_annexes_for_c24() {
        let v_ed_kn = 15.0;
        let b_mm = 200.0;
        let h_mm = 300.0;
        let f_v_k = 4.0;
        let km = k_mod(ServiceClass::Sc1, LoadDuration::Medium);
        let check_en = part_1_1::check_shear(v_ed_kn, b_mm, h_mm, f_v_k, km, AnnexChoice::En);
        let check_de = part_1_1::check_shear(v_ed_kn, b_mm, h_mm, f_v_k, km, AnnexChoice::De);
        assert!((check_en.utilization - check_de.utilization).abs() > 1e-3, "EN and DE shear utilizations must diverge for C24");
        assert!(check_de.utilization > check_en.utilization, "smaller DE k_cr yields a smaller effective width and thus a higher utilization");
        assert!((check_de.utilization - 0.2438).abs() < 0.001);
        assert!((check_en.utilization - 0.2274).abs() < 0.001);
    }

    #[semio_framework_async_macros::async_test]
    fn pedestrian_vibration_within_comfort_limit() {
        let check = part_2::check_pedestrian_vibration(0.3);
        assert!(check.utilization < 1.0);
        let check_exceeding = part_2::check_pedestrian_vibration(1.0);
        assert!(check_exceeding.utilization > 1.0);
    }

    #[semio_framework_async_macros::async_test]
    fn fatigue_reduction_factor_degrades_with_cycles() {
        assert!((part_2::fatigue_reduction_factor(500_000.0) - 1.0).abs() < 1e-9);
        let k_fat = part_2::fatigue_reduction_factor(1.0e8);
        assert!((0.5..1.0).contains(&k_fat));
    }
}
//#endregion 🧪️ComplianceHelpersTests
