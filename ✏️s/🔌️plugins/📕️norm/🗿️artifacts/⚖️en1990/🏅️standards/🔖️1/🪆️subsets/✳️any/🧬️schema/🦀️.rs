//! 🧬️ En1990 artifact schema — every field of the artifact with its state class.

use crate::document::AnnexChoice;
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ Full En1990 artifact state across the artifact and presence lanes.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1990")]
pub struct En1990Artifact {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub project_id: String,
    #[state(artifact)]
    pub structure_kind: String,
    #[state(artifact)]
    pub altitude_m: f64,
    #[state(artifact)]
    pub consequence_class: u8,
    #[state(artifact)]
    pub reliability_class: u8,
    #[state(artifact)]
    pub design_working_life_category: u8,
    #[state(artifact)]
    pub design_working_life_years: f64,
    #[state(artifact)]
    pub reference_period_years: f64,
    #[state(artifact)]
    pub supervision_level: String,
    #[state(artifact)]
    pub inspection_level: String,
    #[state(artifact)]
    pub k_fi_declared: f64,
    #[state(artifact)]
    pub beta_computed: f64,
    #[state(artifact)]
    pub permanents: Vec<crate::PermanentAction>,
    #[state(artifact)]
    pub variables: Vec<crate::VariableAction>,
    #[state(artifact)]
    pub accidentals: Vec<crate::AccidentalAction>,
    #[state(artifact)]
    pub seismics: Vec<crate::SeismicAction>,
    #[state(artifact)]
    pub members: Vec<crate::Member>,
    #[state(artifact)]
    pub bridge_sls: Vec<crate::BridgeSls>,
    #[state(artifact)]
    pub effects: Vec<crate::MemberEffect>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl En1990Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::En1990Snapshot {
        crate::En1990Snapshot {
            annex: self.annex,
            project_id: self.project_id.clone(),
            structure_kind: self.structure_kind.clone(),
            altitude_m: self.altitude_m,
            consequence_class: self.consequence_class,
            reliability_class: self.reliability_class,
            design_working_life_category: self.design_working_life_category,
            design_working_life_years: self.design_working_life_years,
            reference_period_years: self.reference_period_years,
            supervision_level: self.supervision_level.clone(),
            inspection_level: self.inspection_level.clone(),
            k_fi_declared: self.k_fi_declared,
            beta_computed: self.beta_computed,
            permanents: self.permanents.clone(),
            variables: self.variables.clone(),
            accidentals: self.accidentals.clone(),
            seismics: self.seismics.clone(),
            members: self.members.clone(),
            bridge_sls: self.bridge_sls.clone(),
            effects: self.effects.clone(),
        }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: crate::En1990Snapshot) -> Self {
        Self {
            annex: snapshot.annex,
            project_id: snapshot.project_id,
            structure_kind: snapshot.structure_kind,
            altitude_m: snapshot.altitude_m,
            consequence_class: snapshot.consequence_class,
            reliability_class: snapshot.reliability_class,
            design_working_life_category: snapshot.design_working_life_category,
            design_working_life_years: snapshot.design_working_life_years,
            reference_period_years: snapshot.reference_period_years,
            supervision_level: snapshot.supervision_level,
            inspection_level: snapshot.inspection_level,
            k_fi_declared: snapshot.k_fi_declared,
            beta_computed: snapshot.beta_computed,
            permanents: snapshot.permanents,
            variables: snapshot.variables,
            accidentals: snapshot.accidentals,
            seismics: snapshot.seismics,
            members: snapshot.members,
            bridge_sls: snapshot.bridge_sls,
            effects: snapshot.effects,
        }
    }

    /// 🔄 Overwrite persistent fields from a snapshot; leave shared-ui untouched.
    pub fn set_snapshot(&mut self, snapshot: crate::En1990Snapshot) {
        *self = Self::from_snapshot(snapshot);
    }
}
//#endregion 🔖️Conversions



//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.en1990` — twenty handcrafted schema leaves.
pub fn en1990_artifact_schema_descriptor() -> framework_schema::ArtifactSchemaDescriptor {
    framework_schema::ArtifactSchemaDescriptor {
        id: "s.norm.en1990",
        artifact: framework_schema::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: framework_schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: framework_schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: framework_schema::FacetLeaves {
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
    use crate::{En1990Diff, En1990Mutation, En1990Snapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct En1990BuilderConstruction {
        snapshot: En1990Snapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for En1990BuilderConstruction {
        type Snapshot = En1990Snapshot;
        type Mutation = En1990Mutation;
        type Diff = En1990Diff;
        fn empty() -> Self {
            Self { snapshot: En1990Snapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<En1990Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<En1990Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <En1990Mutation as protocol::Mutation<En1990Snapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <En1990Diff as protocol::MutationDiff<En1990Snapshot>>::apply(&diff, &self.snapshot)?;
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
    use crate::En1990Snapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct En1990Parts {
        pub snapshot: Option<En1990Snapshot>,
    }

    pub struct En1990AnalyzerAnalysis;

    impl ArtifactAnalysis for En1990AnalyzerAnalysis {
        type Parts = En1990Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.norm.en1990", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = En1990Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <En1990Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <En1990Snapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec En1990BuilderFacets {
        construction: En1990BuilderConstruction,
        analysis: En1990AnalyzerAnalysis,
        composition: super::super::io::derived_composition::En1990ComposerComposition,
    }
    builder: En1990Builder,
    analyzer: En1990Analyzer,
    composer: En1990Composer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️ComplianceHelpers
/// 📐️ Pure EN 1990 compliance helpers — combinations, partial-factor tables and national-annex
/// implementations are pure over `ActionSet`/`&impl NationalAnnex`. Snapshot-level `evaluate` lives
/// in `💡️inferences`. `na_de`/`na_en` are depended on by sibling EN 199x artifacts.
use crate::document::{
    CheckReport, CheckResult, CheckStatus, ClauseId, DesignSituation, ImposedCategory, LimitState, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef,
};

pub use crate::document::NationalAnnex;

// #region 🔖️PsiTables
/// 📊️ ψ factors for one imposed-load category (EN 1990 Table A1.1 / DIN EN 1990/NA Table NA.A.1.1).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PsiRow {
    pub psi_0: f64,
    pub psi_1: f64,
    pub psi_2: f64,
}

fn psi_row_de(category: &str) -> PsiRow {
    match category {
        "residential" | "A" => PsiRow { psi_0: 0.7, psi_1: 0.5, psi_2: 0.3 },
        "office" | "B" => PsiRow { psi_0: 0.7, psi_1: 0.5, psi_2: 0.3 },
        "congregation" | "C" => PsiRow { psi_0: 0.7, psi_1: 0.7, psi_2: 0.6 },
        "retail" | "D" => PsiRow { psi_0: 0.7, psi_1: 0.7, psi_2: 0.6 },
        "storage" | "E" => PsiRow { psi_0: 1.0, psi_1: 0.9, psi_2: 0.8 },
        "traffic_light" | "F" => PsiRow { psi_0: 0.7, psi_1: 0.7, psi_2: 0.6 },
        "traffic_heavy" | "G" => PsiRow { psi_0: 0.7, psi_1: 0.5, psi_2: 0.3 },
        "roof" | "H" => PsiRow { psi_0: 0.0, psi_1: 0.0, psi_2: 0.0 },
        "snow" => PsiRow { psi_0: 0.5, psi_1: 0.2, psi_2: 0.0 },
        "snow_high" => PsiRow { psi_0: 0.7, psi_1: 0.5, psi_2: 0.2 },
        "wind" => PsiRow { psi_0: 0.6, psi_1: 0.2, psi_2: 0.0 },
        "road_traffic" => PsiRow { psi_0: 0.75, psi_1: 0.40, psi_2: 0.20 },
        "footbridge_crowd" => PsiRow { psi_0: 0.40, psi_1: 0.40, psi_2: 0.0 },
        "rail_traffic" => PsiRow { psi_0: 0.80, psi_1: 0.50, psi_2: 0.20 },
        "temperature" => PsiRow { psi_0: 0.6, psi_1: 0.5, psi_2: 0.0 },
        "settlement" => PsiRow { psi_0: 1.0, psi_1: 1.0, psi_2: 1.0 },
        "other" => PsiRow { psi_0: 0.8, psi_1: 0.7, psi_2: 0.5 },
        _ => PsiRow { psi_0: 0.7, psi_1: 0.5, psi_2: 0.3 },
    }
}

fn psi_row_en(category: &str) -> PsiRow {
    match category {
        "residential" | "A" => PsiRow { psi_0: 0.7, psi_1: 0.5, psi_2: 0.3 },
        "office" | "B" => PsiRow { psi_0: 0.7, psi_1: 0.5, psi_2: 0.3 },
        "congregation" | "C" => PsiRow { psi_0: 0.7, psi_1: 0.7, psi_2: 0.6 },
        "retail" | "D" => PsiRow { psi_0: 0.7, psi_1: 0.7, psi_2: 0.6 },
        "storage" | "E" => PsiRow { psi_0: 1.0, psi_1: 0.9, psi_2: 0.8 },
        "traffic_light" | "F" => PsiRow { psi_0: 0.7, psi_1: 0.7, psi_2: 0.6 },
        "traffic_heavy" | "G" => PsiRow { psi_0: 0.7, psi_1: 0.5, psi_2: 0.3 },
        "roof" | "H" => PsiRow { psi_0: 0.0, psi_1: 0.0, psi_2: 0.0 },
        "snow" => PsiRow { psi_0: 0.5, psi_1: 0.2, psi_2: 0.0 },
        "wind" => PsiRow { psi_0: 0.6, psi_1: 0.2, psi_2: 0.0 },
        "road_traffic" => PsiRow { psi_0: 0.75, psi_1: 0.40, psi_2: 0.20 },
        "footbridge_crowd" => PsiRow { psi_0: 0.40, psi_1: 0.40, psi_2: 0.0 },
        "rail_traffic" => PsiRow { psi_0: 0.80, psi_1: 0.50, psi_2: 0.20 },
        "temperature" => PsiRow { psi_0: 0.6, psi_1: 0.5, psi_2: 0.0 },
        "settlement" => PsiRow { psi_0: 1.0, psi_1: 1.0, psi_2: 1.0 },
        _ => PsiRow { psi_0: 0.7, psi_1: 0.5, psi_2: 0.3 },
    }
}

/// 🌨️ Resolve ψ category: DE-NA snow at altitude > 1000 m uses `snow_high`.
pub fn resolve_psi_category(annex: AnnexChoice, category: &str, altitude_m: f64) -> String {
    if annex == AnnexChoice::De && (category == "snow" || category == "snow_high") && altitude_m > 1000.0 {
        "snow_high".into()
    } else if category == "snow_high" && annex != AnnexChoice::De {
        "snow".into()
    } else {
        category.into()
    }
}

pub fn psi_for_category<A: NationalAnnex>(annex: &A, category: &str) -> PsiRow {
    if annex.choice() == AnnexChoice::De {
        psi_row_de(category)
    } else {
        psi_row_en(category)
    }
}

pub fn psi_for_imposed<A: NationalAnnex>(annex: &A, category: ImposedCategory) -> PsiRow {
    psi_for_category(annex, category.label())
}
// #endregion 🔖️PsiTables

// #region 🔖️NaDe
/// 🇩️🇪️ German national annex parameters (DIN EN 1990/NA).
#[derive(Clone, Copy, Debug, Default)]
pub struct NaDe;

impl NationalAnnex for NaDe {
    fn choice(&self) -> AnnexChoice {
        AnnexChoice::De
    }

    fn gamma_g(&self) -> f64 {
        1.35
    }

    fn gamma_q(&self) -> f64 {
        1.5
    }

    fn gamma_m(&self, material: &str) -> f64 {
        match material {
            "concrete" => 1.5,
            "steel" => 1.0,
            "timber" => 1.3,
            _ => 1.0,
        }
    }

    fn gamma_r(&self) -> f64 {
        1.0
    }

    fn xi(&self, category: &str) -> f64 {
        match category {
            "accidental" | "seismic" => 1.0,
            _ => 0.85,
        }
    }

    fn psi_0(&self, category: &str) -> f64 {
        psi_row_de(category).psi_0
    }

    fn psi_1(&self, category: &str) -> f64 {
        psi_row_de(category).psi_1
    }

    fn psi_2(&self, category: &str) -> f64 {
        psi_row_de(category).psi_2
    }
}
// #endregion 🔖️NaDe

// #region 🔖️NaEn
/// 🇪️🇺️ Recommended values EN 1990.
#[derive(Clone, Copy, Debug, Default)]
pub struct NaEn;

impl NationalAnnex for NaEn {
    fn choice(&self) -> AnnexChoice {
        AnnexChoice::En
    }

    fn gamma_g(&self) -> f64 {
        1.35
    }

    fn gamma_q(&self) -> f64 {
        1.5
    }

    fn gamma_m(&self, material: &str) -> f64 {
        match material {
            "concrete" => 1.5,
            "steel" => 1.0,
            "timber" => 1.3,
            _ => 1.0,
        }
    }

    fn gamma_r(&self) -> f64 {
        1.0
    }

    fn xi(&self, _category: &str) -> f64 {
        0.85
    }

    fn psi_0(&self, category: &str) -> f64 {
        psi_row_en(category).psi_0
    }

    fn psi_1(&self, category: &str) -> f64 {
        psi_row_en(category).psi_1
    }

    fn psi_2(&self, category: &str) -> f64 {
        psi_row_en(category).psi_2
    }
}
// #endregion 🔖️NaEn

// #region 🔖️NationalAnnexes
use semio_framework_dispatch_macros::dyn_enum_close;
use semio_s_artifact_norm_contract::__semio_dispatch_NationalAnnex;
dyn_enum_close! {
    pub enum NationalAnnexes: NationalAnnex {
        De(NaDe),
        En(NaEn),
    }
}
// #endregion 🔖️NationalAnnexes

pub mod na_de {
    pub use super::NaDe;
}

pub mod na_en {
    pub use super::NaEn;
}

// #region 🔖️KfiReliability
/// ⚖️ K_FI consequence-class factor (EN 1990 Annex B / DIN EN 1990/NA).

/// 📅 EN 1990 Table 2.1 indicative design working life [years] for category 1–5 (bridges → 100 a as cat. 5).
pub fn design_working_life_indicative(category: u8) -> (f64, f64) {
    match category {
        1 => (10.0, 10.0),
        2 => (10.0, 25.0),
        3 => (15.0, 30.0),
        4 => (50.0, 50.0),
        5 => (100.0, 100.0),
        _ => (50.0, 50.0),
    }
}

/// 👷 Annex B recommended DSL/IL for reliability class (RC1→1, RC2→2, RC3→3).
pub fn annex_b_dsl_il_for_rc(reliability_class: u8) -> (&'static str, &'static str) {
    match reliability_class {
        1 => ("DSL1", "IL1"),
        3 => ("DSL3", "IL3"),
        _ => ("DSL2", "IL2"),
    }
}

pub fn k_fi(consequence_class: u8) -> f64 {
    match consequence_class {
        1 => 0.9,
        3 => 1.1,
        _ => 1.0,
    }
}

/// 📐️ Target reliability index β from reliability class and reference period (EN 1990 Annex C Table C.2).
pub fn target_reliability_index_for(reliability_class: u8, reference_period_years: f64) -> f64 {
    let fifty = reference_period_years >= 25.0;
    match (reliability_class, fifty) {
        (1, true) => 3.3,
        (1, false) => 4.2,
        (3, true) => 4.3,
        (3, false) => 5.2,
        (_, true) => 3.8,
        (_, false) => 4.7,
    }
}

/// 📐️ Reliability index target β mapped from consequence class (legacy helper — prefer RC+period).
pub fn target_reliability_index(consequence_class: u8) -> f64 {
    target_reliability_index_for(consequence_class, 50.0)
}
// #endregion 🔖️KfiReliability

// #region 🔖️Combinations
/// 📊️ Permanent and variable action components for combination (unit-agnostic: same unit in/out).
#[derive(Clone, Debug, PartialEq, Default)]
pub struct ActionSet {
    pub g_k: f64,
    pub g_k_inf: f64,
    pub p_k: f64,
    pub q_k: Vec<(String, f64)>,
    pub a_d: f64,
    pub a_ed: f64,
}

impl ActionSet {
    pub fn new(g_k: f64, q_k: Vec<(String, f64)>) -> Self {
        Self { g_k, q_k, ..Default::default() }
    }
}

/// 🏷️ ULS/SLS combination rule identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CombinationRule {
    Uls610,
    Uls610a,
    Uls610b,
    Uls611,
    Uls612,
    Uls612b,
    SlsCharacteristic,
    SlsFrequent,
    SlsQuasiPermanent,
}

/// ⚖️ EQU vs STR/GEO partial-factor set (EN 1990 Tables A1.2(A)/(B)).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PartialFactorSet {
    EquA12A,
    StrGeoA12B,
    EquA24A,
    StrGeoA24B,
    GeoA24C,
}

fn gamma_g_set(set: PartialFactorSet, favourable: bool) -> f64 {
    match (set, favourable) {
        (PartialFactorSet::EquA12A, false) => 1.10,
        (PartialFactorSet::EquA12A, true) => 0.90,
        (PartialFactorSet::StrGeoA12B, false) => 1.35,
        (PartialFactorSet::StrGeoA12B, true) => 1.00,
        (PartialFactorSet::EquA24A, false) => 1.05,
        (PartialFactorSet::EquA24A, true) => 0.95,
        (PartialFactorSet::StrGeoA24B, false) => 1.35,
        (PartialFactorSet::StrGeoA24B, true) => 1.00,
        (PartialFactorSet::GeoA24C, false) => 1.00,
        (PartialFactorSet::GeoA24C, true) => 1.00,
    }
}

fn gamma_q_set(set: PartialFactorSet) -> f64 {
    match set {
        PartialFactorSet::EquA12A => 1.50,
        PartialFactorSet::StrGeoA12B => 1.50,
        PartialFactorSet::EquA24A => 1.35,
        PartialFactorSet::StrGeoA24B => 1.50,
        PartialFactorSet::GeoA24C => 1.30,
    }
}

/// 🌉 Annex A2.4(B)/(C) γ_Q for bridge traffic categories (DE NA vs EN recommended).
pub fn gamma_q_bridge_traffic(annex: AnnexChoice, category: &str) -> f64 {
    match (annex, category) {
        (AnnexChoice::De, "road_traffic") => 1.35,
        (AnnexChoice::En, "road_traffic") => 1.35,
        (AnnexChoice::De, "footbridge_crowd") => 1.50,
        (AnnexChoice::En, "footbridge_crowd") => 1.50,
        (AnnexChoice::De, "rail_traffic") => 1.40,
        (AnnexChoice::En, "rail_traffic") => 1.45,
        (AnnexChoice::De, _) => 1.50,
        (AnnexChoice::En, _) => 1.50,
    }
}

fn is_bridge_kind(kind: &str) -> bool {
    matches!(kind, "road_bridge" | "footbridge" | "rail_bridge")
}

fn str_geo_set(structure_kind: &str) -> PartialFactorSet {
    if is_bridge_kind(structure_kind) { PartialFactorSet::StrGeoA24B } else { PartialFactorSet::StrGeoA12B }
}

fn equ_set(structure_kind: &str) -> PartialFactorSet {
    if is_bridge_kind(structure_kind) { PartialFactorSet::EquA24A } else { PartialFactorSet::EquA12A }
}

fn gamma_q_for_action(annex: AnnexChoice, structure_kind: &str, set: PartialFactorSet, category: &str) -> f64 {
    if is_bridge_kind(structure_kind) && matches!(category, "road_traffic" | "footbridge_crowd" | "rail_traffic") {
        gamma_q_bridge_traffic(annex, category)
    } else {
        gamma_q_set(set)
    }
}

fn gamma_for_situation<A: NationalAnnex>(annex: &A, situation: DesignSituation) -> (f64, f64) {
    match situation {
        DesignSituation::Persistent | DesignSituation::Transient => (annex.gamma_g(), annex.gamma_q()),
        DesignSituation::Accidental | DesignSituation::Seismic => (1.0, 1.0),
    }
}

fn xi_for_situation<A: NationalAnnex>(annex: &A, situation: DesignSituation) -> f64 {
    match situation {
        DesignSituation::Persistent | DesignSituation::Transient => annex.xi("permanent"),
        DesignSituation::Accidental => annex.xi("accidental"),
        DesignSituation::Seismic => annex.xi("seismic"),
    }
}

fn permanent_unfav(actions: &ActionSet) -> f64 {
    actions.g_k + actions.p_k
}

/// 🧮️ ULS combination per EN 1990 Eq. 6.10: max(6.10a, 6.10b).
pub fn combination_6_10<A: NationalAnnex>(annex: &A, actions: &ActionSet, leading: usize) -> f64 {
    combination_6_10a(annex, actions, leading).max(combination_6_10b(annex, actions, leading))
}

/// 🧮️ ULS combination per EN 1990 Eq. 6.10a: γ_G·G + γ_Q·Q + γ_Q·ψ_0·ΣQ.
pub fn combination_6_10a<A: NationalAnnex>(annex: &A, actions: &ActionSet, leading: usize) -> f64 {
    let mut sum = annex.gamma_g() * permanent_unfav(actions);
    for (i, (cat, q)) in actions.q_k.iter().enumerate() {
        let factor = if i == leading { annex.gamma_q() } else { annex.gamma_q() * annex.psi_0(cat) };
        sum += factor * q;
    }
    sum
}

/// 🧮️ ULS combination per EN 1990 Eq. 6.10b: ξ·γ_G·G + γ_Q·Q + γ_Q·ψ_0·ΣQ.
pub fn combination_6_10b<A: NationalAnnex>(annex: &A, actions: &ActionSet, leading: usize) -> f64 {
    let xi = annex.xi("permanent");
    let mut sum = xi * annex.gamma_g() * permanent_unfav(actions);
    for (i, (cat, q)) in actions.q_k.iter().enumerate() {
        let factor = if i == leading { annex.gamma_q() } else { annex.gamma_q() * annex.psi_0(cat) };
        sum += factor * q;
    }
    sum
}

/// 🧮️ ULS STR/GEO with K_FI on γ (Annex B) for set A1.2(B).
pub fn combination_str_geo_610a<A: NationalAnnex>(annex: &A, actions: &ActionSet, leading: usize, consequence_class: u8) -> f64 {
    combination_str_geo_610a_for(annex, actions, leading, consequence_class, "building")
}

/// 🌉 STR/GEO 6.10a with Annex A1.2(B) or A2.4(B) action γ depending on structure kind.
/// 📐 STR/GEO Eq. 6.10a. Governing STR/GEO uses max(6.10a, 6.10b) per DIN EN 1990/NA NDP A1.3.1(4).
pub fn combination_str_geo_610a_for<A: NationalAnnex>(annex: &A, actions: &ActionSet, leading: usize, consequence_class: u8, structure_kind: &str) -> f64 {
    let k = k_fi(consequence_class);
    let set = str_geo_set(structure_kind);
    let gamma_g_sup = k * gamma_g_set(set, false);
    let gamma_g_inf = k * gamma_g_set(set, true);
    let mut sum = gamma_g_sup * permanent_unfav(actions) + gamma_g_inf * actions.g_k_inf;
    for (i, (cat, q)) in actions.q_k.iter().enumerate() {
        let gq = k * gamma_q_for_action(annex.choice(), structure_kind, set, cat);
        let factor = if i == leading { gq } else { gq * annex.psi_0(cat) };
        sum += factor * q;
    }
    sum
}

pub fn combination_str_geo_610b<A: NationalAnnex>(annex: &A, actions: &ActionSet, leading: usize, consequence_class: u8) -> f64 {
    combination_str_geo_610b_for(annex, actions, leading, consequence_class, "building")
}

/// 🌉 STR/GEO 6.10b with Annex A1.2(B) or A2.4(B) action γ depending on structure kind.
pub fn combination_str_geo_610b_for<A: NationalAnnex>(annex: &A, actions: &ActionSet, leading: usize, consequence_class: u8, structure_kind: &str) -> f64 {
    let k = k_fi(consequence_class);
    let xi = annex.xi("permanent");
    let set = str_geo_set(structure_kind);
    let gamma_g_sup = k * gamma_g_set(set, false);
    let gamma_g_inf = k * gamma_g_set(set, true);
    let mut sum = xi * gamma_g_sup * permanent_unfav(actions) + gamma_g_inf * actions.g_k_inf;
    for (i, (cat, q)) in actions.q_k.iter().enumerate() {
        let gq = k * gamma_q_for_action(annex.choice(), structure_kind, set, cat);
        let factor = if i == leading { gq } else { gq * annex.psi_0(cat) };
        sum += factor * q;
    }
    sum
}

/// 🌉 GEO set C (Table A2.4(C)): γ_G = 1.0, γ_Q from set C / traffic.
pub fn combination_geo_a24c<A: NationalAnnex>(annex: &A, actions: &ActionSet, leading: usize, consequence_class: u8, structure_kind: &str) -> f64 {
    let k = k_fi(consequence_class);
    let set = PartialFactorSet::GeoA24C;
    let gamma_g = k * gamma_g_set(set, false);
    let mut sum = gamma_g * (permanent_unfav(actions) + actions.g_k_inf);
    for (i, (cat, q)) in actions.q_k.iter().enumerate() {
        let gq = k * gamma_q_for_action(annex.choice(), structure_kind, set, cat);
        let factor = if i == leading { gq } else { gq * annex.psi_0(cat) };
        sum += factor * q;
    }
    sum
}

/// ⚖️ EQU A1.2(A): destabilising vs stabilising permanent with γ_G,dstab / γ_G,stab.
pub fn combination_equ_a12a<A: NationalAnnex>(annex: &A, actions: &ActionSet, leading: usize, consequence_class: u8) -> f64 {
    combination_equ_for(annex, actions, leading, consequence_class, "building")
}

/// ⚖️ EQU with A1.2(A) or A2.4(A) action γ depending on structure kind.
pub fn combination_equ_for<A: NationalAnnex>(annex: &A, actions: &ActionSet, leading: usize, consequence_class: u8, structure_kind: &str) -> f64 {
    let k = k_fi(consequence_class);
    let set = equ_set(structure_kind);
    let g_dstab = k * gamma_g_set(set, false) * permanent_unfav(actions);
    let g_stab = k * gamma_g_set(set, true) * actions.g_k_inf;
    let mut q_sum = 0.0;
    for (i, (cat, q)) in actions.q_k.iter().enumerate() {
        let gq = k * gamma_q_for_action(annex.choice(), structure_kind, set, cat);
        let factor = if i == leading { gq } else { gq * annex.psi_0(cat) };
        q_sum += factor * q;
    }
    g_dstab + q_sum - g_stab
}

/// 🧮️ ULS combination for a design situation with situation-specific γ factors.
pub fn combination_uls<A: NationalAnnex>(annex: &A, situation: DesignSituation, rule: CombinationRule, actions: &ActionSet, leading: usize) -> f64 {
    let (gamma_g, gamma_q) = gamma_for_situation(annex, situation);
    let xi = xi_for_situation(annex, situation);
    match rule {
        CombinationRule::Uls610 => combination_uls(annex, situation, CombinationRule::Uls610a, actions, leading)
            .max(combination_uls(annex, situation, CombinationRule::Uls610b, actions, leading)),
        CombinationRule::Uls611 => combination_6_11(annex, actions, leading),
        CombinationRule::Uls612 | CombinationRule::Uls612b => combination_6_12b(annex, actions, actions.a_ed),
        _ => {
            let g_factor = match rule {
                CombinationRule::Uls610b => xi * gamma_g,
                _ => gamma_g,
            };
            let mut sum = g_factor * permanent_unfav(actions);
            for (i, (cat, q)) in actions.q_k.iter().enumerate() {
                let factor = if i == leading { gamma_q } else { gamma_q * annex.psi_0(cat) };
                sum += factor * q;
            }
            sum
        }
    }
}

/// 💥 Accidental combination EN 1990 Eq. 6.11: ΣG + A_d + ψ_{1,1} Q_{k,1} + Σψ_{2,i} Q_{k,i}.
pub fn combination_6_11<A: NationalAnnex>(annex: &A, actions: &ActionSet, leading: usize) -> f64 {
    let mut sum = permanent_unfav(actions) + actions.a_d;
    for (i, (cat, q)) in actions.q_k.iter().enumerate() {
        let factor = if i == leading { annex.psi_1(cat) } else { annex.psi_2(cat) };
        sum += factor * q;
    }
    sum
}

/// 🌋️ Seismic combination EN 1990 Eq. 6.12b: ΣG_k + A_Ed + Σψ_2·Q_k.
pub fn combination_6_12b<A: NationalAnnex>(annex: &A, actions: &ActionSet, seismic_a_ed: f64) -> f64 {
    let mut sum = permanent_unfav(actions) + seismic_a_ed;
    for (cat, q) in &actions.q_k {
        sum += annex.psi_2(cat) * q;
    }
    sum
}

/// 🧮️ SLS characteristic combination: G + Q + ψ_0·ΣQ.
pub fn combination_sls_char<A: NationalAnnex>(annex: &A, actions: &ActionSet, leading: usize) -> f64 {
    let mut sum = permanent_unfav(actions) + actions.g_k_inf;
    for (i, (cat, q)) in actions.q_k.iter().enumerate() {
        let factor = if i == leading { 1.0 } else { annex.psi_0(cat) };
        sum += factor * q;
    }
    sum
}

/// 🧮️ SLS frequent combination: G + ψ_1·Q_leading + ψ_2·ΣQ_accompanying.
pub fn combination_sls_frequent<A: NationalAnnex>(annex: &A, actions: &ActionSet, leading: usize) -> f64 {
    let mut sum = permanent_unfav(actions) + actions.g_k_inf;
    for (i, (cat, q)) in actions.q_k.iter().enumerate() {
        let factor = if i == leading { annex.psi_1(cat) } else { annex.psi_2(cat) };
        sum += factor * q;
    }
    sum
}

/// 🧮️ SLS quasi-permanent combination: G + ψ_2·ΣQ.
pub fn combination_sls_quasi_permanent<A: NationalAnnex>(annex: &A, actions: &ActionSet) -> f64 {
    let mut sum = permanent_unfav(actions) + actions.g_k_inf;
    for (cat, q) in &actions.q_k {
        sum += annex.psi_2(cat) * q;
    }
    sum
}

pub fn combination_value<A: NationalAnnex>(annex: &A, rule: CombinationRule, actions: &ActionSet, leading: usize) -> f64 {
    match rule {
        CombinationRule::Uls610 => combination_6_10(annex, actions, leading),
        CombinationRule::Uls610a => combination_6_10a(annex, actions, leading),
        CombinationRule::Uls610b => combination_6_10b(annex, actions, leading),
        CombinationRule::Uls611 => combination_6_11(annex, actions, leading),
        CombinationRule::Uls612 | CombinationRule::Uls612b => combination_6_12b(annex, actions, actions.a_ed),
        CombinationRule::SlsCharacteristic => combination_sls_char(annex, actions, leading),
        CombinationRule::SlsFrequent => combination_sls_frequent(annex, actions, leading),
        CombinationRule::SlsQuasiPermanent => combination_sls_quasi_permanent(annex, actions),
    }
}

/// 📋️ Combination rules relevant for a design situation and limit state.
pub fn rules_for_situation(situation: DesignSituation, limit_state: LimitState) -> Vec<CombinationRule> {
    match (situation, limit_state) {
        (DesignSituation::Persistent | DesignSituation::Transient, LimitState::Uls) => {
            vec![CombinationRule::Uls610, CombinationRule::Uls610a, CombinationRule::Uls610b]
        }
        (DesignSituation::Accidental, LimitState::Uls) => vec![CombinationRule::Uls611],
        (DesignSituation::Seismic, LimitState::Uls) => vec![CombinationRule::Uls612b],
        (_, LimitState::Sls) => vec![CombinationRule::SlsCharacteristic, CombinationRule::SlsFrequent, CombinationRule::SlsQuasiPermanent],
        (_, LimitState::Als) => vec![CombinationRule::Uls611],
        (_, LimitState::Fls) => vec![CombinationRule::Uls610a],
    }
}

fn clause_for_rule(rule: CombinationRule) -> ClauseId {
    match rule {
        CombinationRule::Uls610 => ClauseId::new("EN 1990", "§6.4", "6.10"),
        CombinationRule::Uls610a => ClauseId::new("EN 1990", "§6.4", "6.10a"),
        CombinationRule::Uls610b => ClauseId::new("EN 1990", "§6.4", "6.10b"),
        CombinationRule::Uls611 => ClauseId::new("EN 1990", "§6.4", "6.11"),
        CombinationRule::Uls612 => ClauseId::new("EN 1990", "§6.4.3.4", "6.12"),
        CombinationRule::Uls612b => ClauseId::new("EN 1990", "§6.4.3.4", "6.12b"),
        CombinationRule::SlsCharacteristic => ClauseId::new("EN 1990", "§6.5", "6.14"),
        CombinationRule::SlsFrequent => ClauseId::new("EN 1990", "§6.5", "6.16"),
        CombinationRule::SlsQuasiPermanent => ClauseId::new("EN 1990", "§6.5", "6.17"),
    }
}

fn copy(en: impl Into<String>, de: impl Into<String>) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

fn q_force(n: f64) -> Quantity {
    Quantity::new(QuantityKind::Force, n)
}

fn q_dim(v: f64) -> Quantity {
    Quantity::new(QuantityKind::Dimensionless, v)
}

fn resistance_remedy(member_id: &str, path: &str, label: LocalizedCopy, current: f64, required: f64) -> Remedy {
    let kn = required / 1000.0;
    let cur_kn = current / 1000.0;
    Remedy::at_least(
        SubjectRef::new(member_id, path, label),
        q_force(current),
        q_force(required),
        copy(
            format!("Increase design resistance from {cur_kn:.1} kN to at least {kn:.1} kN."),
            format!("Bemessungswiderstand von {cur_kn:.1} kN auf mindestens {kn:.1} kN erhöhen."),
        ),
    )
}

/// ✅️ Check one combination against a resistance limit (same unit as ActionSet).
pub fn check_combination<A: NationalAnnex>(annex: &A, situation: DesignSituation, rule: CombinationRule, actions: &ActionSet, leading: usize, resistance: f64) -> CheckResult {
    let ed = if matches!(rule, CombinationRule::Uls610 | CombinationRule::Uls610a | CombinationRule::Uls610b | CombinationRule::Uls611 | CombinationRule::Uls612 | CombinationRule::Uls612b) {
        combination_uls(annex, situation, rule, actions, leading)
    } else {
        combination_value(annex, rule, actions, leading)
    };
    let id = format!("en1990.{}.{leading}", rule_slug(rule));
    let title = copy(format!("Combination {}", clause_for_rule(rule).section), format!("Kombination {}", clause_for_rule(rule).section));
    let mut builder = CheckResult::assess(id, "EN 1990", clause_for_rule(rule), SubjectRef::whole(copy("Structure", "Tragwerk")), title)
        .utilization(q_force(ed), q_force(resistance))
        .annex(annex.choice())
        .explanation(copy(
            format!("E_d = {ed:.3} vs R_d = {resistance:.3} (leading={leading})"),
            format!("E_d = {ed:.3} gegenüber R_d = {resistance:.3} (führend={leading})"),
        ));
    if ed > resistance {
        builder = builder.remedy(resistance_remedy("", "members[id=governing].rdStr", copy("Governing member", "Maßgebendes Bauteil"), resistance, ed));
    }
    builder.build()
}

fn rule_slug(rule: CombinationRule) -> &'static str {
    match rule {
        CombinationRule::Uls610 => "6.10",
        CombinationRule::Uls610a => "6.10a",
        CombinationRule::Uls610b => "6.10b",
        CombinationRule::Uls611 => "6.11",
        CombinationRule::Uls612 => "6.12",
        CombinationRule::Uls612b => "6.12b",
        CombinationRule::SlsCharacteristic => "6.14b",
        CombinationRule::SlsFrequent => "6.15b",
        CombinationRule::SlsQuasiPermanent => "6.16b",
    }
}

/// ✅️ Run all relevant combinations for an action set in a design situation.
pub fn check_combination_set<A: NationalAnnex>(annex: &A, situation: DesignSituation, actions: &ActionSet, resistance: f64) -> CheckReport {
    let mut report = CheckReport::default();
    let n_leading = actions.q_k.len().max(1);
    for rule in rules_for_situation(situation, LimitState::Uls) {
        for leading in 0..n_leading {
            if actions.q_k.is_empty() && leading > 0 {
                break;
            }
            report.push(check_combination(annex, situation, rule, actions, leading, resistance));
        }
    }
    for rule in rules_for_situation(situation, LimitState::Sls) {
        match rule {
            CombinationRule::SlsQuasiPermanent => {
                report.push(check_combination(annex, situation, rule, actions, 0, resistance));
            }
            _ => {
                for leading in 0..n_leading {
                    if actions.q_k.is_empty() && leading > 0 {
                        break;
                    }
                    report.push(check_combination(annex, situation, rule, actions, leading, resistance));
                }
            }
        }
    }
    report
}

/// ✅️ Check design action against resistance (ULS Eq. 6.10 governing).
pub fn check_uls_action<A: NationalAnnex>(annex: &A, actions: &ActionSet, leading: usize, resistance: f64) -> CheckResult {
    let ed = combination_6_10(annex, actions, leading);
    let mut builder = CheckResult::assess("en1990.6.10.uls", "EN 1990", ClauseId::new("EN 1990", "§6.4", "6.10"), SubjectRef::whole(copy("Structure", "Tragwerk")), copy("ULS design action", "Grenzzustand der Tragfähigkeit"))
        .utilization(q_force(ed), q_force(resistance))
        .annex(annex.choice())
        .explanation(copy(format!("Governing E_d = max(6.10a, 6.10b) = {ed:.3}"), format!("Maßgebendes E_d = max(6.10a, 6.10b) = {ed:.3}")));
    if ed > resistance {
        builder = builder.remedy(resistance_remedy("", "members[id=governing].rdStr", copy("Governing member", "Maßgebendes Bauteil"), resistance, ed));
    }
    builder.build()
}
// #endregion 🔖️Combinations

// #region 🔖️Reliability
pub fn check_reliability_index(beta: f64, consequence_class: u8) -> CheckResult {
    check_reliability_index_for(beta, consequence_class, consequence_class, 50.0, AnnexChoice::En)
}

pub fn check_reliability_index_for(beta: f64, consequence_class: u8, reliability_class: u8, reference_period_years: f64, annex: AnnexChoice) -> CheckResult {
    let target = target_reliability_index_for(reliability_class, reference_period_years);
    let mut builder = CheckResult::assess(
        "en1990.annex-c.beta",
        "EN 1990 Annex C",
        ClauseId::new("EN 1990", "Annex C", "C.2"),
        SubjectRef::new("", "betaComputed", copy("Reliability index", "Zuverlässigkeitsindex")),
        copy("Target reliability index β", "Zielwert des Zuverlässigkeitsindex β"),
    )
    .minimum(q_dim(beta), q_dim(target))
    .annex(annex)
    .explanation(copy(
        format!("β = {beta:.2} vs β_target = {target:.2} (RC{reliability_class}, T = {reference_period_years:.0} a, CC{consequence_class})"),
        format!("β = {beta:.2} gegenüber β_Ziel = {target:.2} (RC{reliability_class}, T = {reference_period_years:.0} a, CC{consequence_class})"),
    ));
    if beta < target {
        builder = builder.remedy(Remedy::at_least(
            SubjectRef::new("", "betaComputed", copy("Reliability index", "Zuverlässigkeitsindex")),
            q_dim(beta),
            q_dim(target),
            copy(
                format!("Increase computed β from {beta:.2} to at least {target:.2}, or lower the reliability class."),
                format!("Berechnetes β von {beta:.2} auf mindestens {target:.2} erhöhen oder Zuverlässigkeitsklasse absenken."),
            ),
        ));
    }
    builder.build()
}
// #endregion 🔖️Reliability

/// 🔁️ Append one design-situation's combination checks onto a shared report.
pub fn append_combination_set<A: NationalAnnex>(report: &mut CheckReport, annex: &A, situation: DesignSituation, actions: &ActionSet, resistance: f64) {
    let sub = check_combination_set(annex, situation, actions, resistance);
    report.extend(sub.checks);
}

/// 📋️ Run EN 1990 design basis checks across persistent, accidental, and seismic situations.
pub fn check_design_basis<A: NationalAnnex>(annex: &A, actions: &ActionSet, resistance: f64, consequence_class: u8) -> CheckReport {
    let mut report = CheckReport::default();
    append_combination_set(&mut report, annex, DesignSituation::Persistent, actions, resistance);
    append_combination_set(&mut report, annex, DesignSituation::Accidental, actions, resistance);
    append_combination_set(&mut report, annex, DesignSituation::Seismic, actions, resistance);
    report.push(check_reliability_index_for(3.9, consequence_class, consequence_class, 50.0, annex.choice()));
    report
}

/// ✅️ Check the seismic design situation per EN 1990 Eq. 6.12b.
pub fn check_seismic_situation<A: NationalAnnex>(annex: &A, actions: &ActionSet, seismic_a_ed: f64, resistance: f64) -> CheckResult {
    if seismic_a_ed.abs() < f64::EPSILON {
        return CheckResult::assess(
            "en1990.6.12b",
            "EN 1990",
            ClauseId::new("EN 1990", "§6.4.3.4", "6.12b"),
            SubjectRef::whole(copy("Structure", "Tragwerk")),
            copy("Seismic design situation", "Erdbebenbemessungssituation"),
        )
        .not_applicable(copy("No seismic action A_Ed defined.", "Keine seismische Einwirkung A_Ed definiert."))
        .annex(annex.choice())
        .build();
    }
    let ed = combination_6_12b(annex, actions, seismic_a_ed);
    let mut builder = CheckResult::assess(
        "en1990.6.12b",
        "EN 1990",
        ClauseId::new("EN 1990", "§6.4.3.4", "6.12b"),
        SubjectRef::whole(copy("Structure", "Tragwerk")),
        copy("Seismic design situation", "Erdbebenbemessungssituation"),
    )
    .utilization(q_force(ed), q_force(resistance))
    .annex(annex.choice())
    .explanation(copy(format!("E_d = G + A_Ed + Σψ₂Q = {ed:.3}"), format!("E_d = G + A_Ed + Σψ₂Q = {ed:.3}")));
    if ed > resistance {
        builder = builder.remedy(resistance_remedy("", "members[id=governing].rdStr", copy("Governing member", "Maßgebendes Bauteil"), resistance, ed));
    }
    builder.build()
}
//#endregion 🔖️ComplianceHelpers

//#region 🧪️ComplianceTests
#[cfg(test)]
#[path = "🧪️tests/⚖️compliance/🦀️.rs"]
mod compliance_tests;
//#endregion 🧪️ComplianceTests
