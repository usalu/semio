//! 🧬️ En1990 artifact schema — every field of the artifact with its state class.

use crate::En1990QkChild;
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ Full En1990 artifact state across the artifact and presence lanes. `q_k` mirrors
/// `En1990Snapshot`'s composed `s.stdio.semio.table` child slot (ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM round 2) — `to_snapshot`/`from_snapshot` copy the
/// handle across verbatim, same as `➗️mathematical`'s `EquationArtifact`.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1990")]
pub struct En1990Artifact {
    #[state(artifact)]
    pub g_k: f64,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.table")]
    #[cfg_attr(test, serde(with = "crate::document::child_identity_oracle"))]
    pub q_k: En1990QkChild,
    #[state(artifact)]
    pub resistance_kn: f64,
    #[state(artifact)]
    pub consequence_class: u8,
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub seismic_a_ed_kn: f64,
    #[state(presence)]
    pub selected_check_index: Option<u32>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl En1990Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::En1990Snapshot {
        crate::En1990Snapshot { g_k: self.g_k, q_k: self.q_k.clone(), resistance_kn: self.resistance_kn, consequence_class: self.consequence_class, annex: self.annex, seismic_a_ed_kn: self.seismic_a_ed_kn }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::En1990Snapshot) -> Self {
        Self { g_k: snapshot.g_k, q_k: snapshot.q_k, resistance_kn: snapshot.resistance_kn, consequence_class: snapshot.consequence_class, annex: snapshot.annex, seismic_a_ed_kn: snapshot.seismic_a_ed_kn, selected_check_index: None }
    }
    /// 🔄 Overwrite persistent fields from a snapshot; leave shared-ui untouched.
    pub fn set_snapshot(&mut self, snapshot: crate::En1990Snapshot) {
        let selected = self.selected_check_index;
        *self = Self::from_snapshot(snapshot);
        self.selected_check_index = selected;
    }
}

//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.en1990` — twenty handcrafted schema leaves.
pub fn en1990_artifact_schema_descriptor() -> framework_schema::ArtifactSchemaDescriptor {
    framework_schema::ArtifactSchemaDescriptor {
        id: "s.norm.en1990",
        artifact: framework_schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
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
/// 📐️ Pure EN 1990 compliance helpers (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// relocated verbatim from the deleted `⚙️engine`. Combinations, partial-factor tables and the
/// national-annex implementations are all pure over `ActionSet`/`&impl NationalAnnex` (O1 de-dyn:
/// generic over the closed `NationalAnnex` set — `NationalAnnexes` below for the runtime-chosen case),
/// never over the
/// whole `En1990Snapshot`; the snapshot-level composition (`evaluate`) lives in `💡️inferences`.
/// `na_de`/`na_en` are depended on by several sibling EN 199x artifacts
/// (`semio_s_artifact_norm_en199x::standards::v1::subsets::any::schema::na_de::NaDe`).
use crate::document::{AnnexChoice, CheckReport, CheckResult, CheckStatus, ClauseId, DesignSituation, ImposedCategory, LimitState, Quantity};

pub use crate::document::NationalAnnex;

// #region 🔖️PsiTables
/// 📊️ ψ factors for one imposed-load category (EN 1990 Table A1.1 / DIN EN 1990/NA Table NA.A.1.1).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PsiRow {
    psi_0: f64,
    psi_1: f64,
    psi_2: f64,
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
        "temperature" => PsiRow { psi_0: 0.6, psi_1: 0.5, psi_2: 0.0 },
        "settlement" => PsiRow { psi_0: 1.0, psi_1: 1.0, psi_2: 1.0 },
        _ => PsiRow { psi_0: 0.7, psi_1: 0.5, psi_2: 0.3 },
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

// #region 🔖️Combinations
/// 📊️ Permanent and variable action components for combination [kN].
#[derive(Clone, Debug, PartialEq)]
pub struct ActionSet {
    pub g_k: f64,
    pub q_k: Vec<(String, f64)>,
}

/// 🏷️ ULS/SLS combination rule identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CombinationRule {
    Uls610,
    Uls610a,
    Uls610b,
    SlsCharacteristic,
    SlsFrequent,
    SlsQuasiPermanent,
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

/// 🧮️ ULS combination per EN 1990 Eq. 6.10: max(6.10a, 6.10b) surrogate as 6.10a.
pub fn combination_6_10<A: NationalAnnex>(annex: &A, actions: &ActionSet, leading: usize) -> f64 {
    combination_6_10a(annex, actions, leading)
}

/// 🧮️ ULS combination per EN 1990 Eq. 6.10a: γ_G·G + γ_Q·Q + γ_Q·ψ_0·ΣQ.
pub fn combination_6_10a<A: NationalAnnex>(annex: &A, actions: &ActionSet, leading: usize) -> f64 {
    let mut sum = annex.gamma_g() * actions.g_k;
    for (i, (cat, q)) in actions.q_k.iter().enumerate() {
        let factor = if i == leading { annex.gamma_q() } else { annex.gamma_q() * annex.psi_0(cat) };
        sum += factor * q;
    }
    sum
}

/// 🧮️ ULS combination per EN 1990 Eq. 6.10b: ξ·γ_G·G + γ_Q·Q + γ_Q·ψ_0·ΣQ.
pub fn combination_6_10b<A: NationalAnnex>(annex: &A, actions: &ActionSet, leading: usize) -> f64 {
    let xi = annex.xi("permanent");
    let mut sum = xi * annex.gamma_g() * actions.g_k;
    for (i, (cat, q)) in actions.q_k.iter().enumerate() {
        let factor = if i == leading { annex.gamma_q() } else { annex.gamma_q() * annex.psi_0(cat) };
        sum += factor * q;
    }
    sum
}

/// 🧮️ ULS combination for a design situation with situation-specific γ factors.
pub fn combination_uls<A: NationalAnnex>(annex: &A, situation: DesignSituation, rule: CombinationRule, actions: &ActionSet, leading: usize) -> f64 {
    let (gamma_g, gamma_q) = gamma_for_situation(annex, situation);
    let xi = xi_for_situation(annex, situation);
    let g_factor = match rule {
        CombinationRule::Uls610b => xi * gamma_g,
        _ => gamma_g,
    };
    let mut sum = g_factor * actions.g_k;
    for (i, (cat, q)) in actions.q_k.iter().enumerate() {
        let factor = if i == leading { gamma_q } else { gamma_q * annex.psi_0(cat) };
        sum += factor * q;
    }
    sum
}

/// 🧮️ SLS characteristic combination: G + Q + ψ_0·ΣQ.
pub fn combination_sls_char<A: NationalAnnex>(annex: &A, actions: &ActionSet, leading: usize) -> f64 {
    let mut sum = actions.g_k;
    for (i, (cat, q)) in actions.q_k.iter().enumerate() {
        let factor = if i == leading { 1.0 } else { annex.psi_0(cat) };
        sum += factor * q;
    }
    sum
}

/// 🧮️ SLS frequent combination: G + ψ_1·Q_leading + ψ_2·ΣQ_accompanying.
pub fn combination_sls_frequent<A: NationalAnnex>(annex: &A, actions: &ActionSet, leading: usize) -> f64 {
    let mut sum = actions.g_k;
    for (i, (cat, q)) in actions.q_k.iter().enumerate() {
        let factor = if i == leading { annex.psi_1(cat) } else { annex.psi_2(cat) };
        sum += factor * q;
    }
    sum
}

/// 🧮️ SLS quasi-permanent combination: G + ψ_2·ΣQ.
pub fn combination_sls_quasi_permanent<A: NationalAnnex>(annex: &A, actions: &ActionSet) -> f64 {
    let mut sum = actions.g_k;
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
        (DesignSituation::Accidental | DesignSituation::Seismic, LimitState::Uls) => {
            vec![CombinationRule::Uls610a]
        }
        (_, LimitState::Sls) => vec![CombinationRule::SlsCharacteristic, CombinationRule::SlsFrequent, CombinationRule::SlsQuasiPermanent],
        (_, LimitState::Als) => vec![CombinationRule::Uls610a],
        (_, LimitState::Fls) => vec![CombinationRule::Uls610a],
    }
}

fn clause_for_rule(rule: CombinationRule) -> ClauseId {
    match rule {
        CombinationRule::Uls610 => ClauseId::new("EN 1990", "§6.4", "6.10"),
        CombinationRule::Uls610a => ClauseId::new("EN 1990", "§6.4", "6.10a"),
        CombinationRule::Uls610b => ClauseId::new("EN 1990", "§6.4", "6.10b"),
        CombinationRule::SlsCharacteristic => ClauseId::new("EN 1990", "§6.5", "6.14"),
        CombinationRule::SlsFrequent => ClauseId::new("EN 1990", "§6.5", "6.16"),
        CombinationRule::SlsQuasiPermanent => ClauseId::new("EN 1990", "§6.5", "6.17"),
    }
}

fn message_for_rule(rule: CombinationRule, leading: usize) -> String {
    match rule {
        CombinationRule::Uls610 => format!("ULS 6.10 leading={leading}"),
        CombinationRule::Uls610a => format!("ULS 6.10a leading={leading}"),
        CombinationRule::Uls610b => format!("ULS 6.10b leading={leading}"),
        CombinationRule::SlsCharacteristic => format!("SLS characteristic leading={leading}"),
        CombinationRule::SlsFrequent => format!("SLS frequent leading={leading}"),
        CombinationRule::SlsQuasiPermanent => "SLS quasi-permanent".into(),
    }
}

/// ✅️ Check one combination against a resistance limit [kN].
pub fn check_combination<A: NationalAnnex>(annex: &A, situation: DesignSituation, rule: CombinationRule, actions: &ActionSet, leading: usize, resistance_kn: f64) -> CheckResult {
    let ed = if matches!(rule, CombinationRule::Uls610 | CombinationRule::Uls610a | CombinationRule::Uls610b) { combination_uls(annex, situation, rule, actions, leading) } else { combination_value(annex, rule, actions, leading) };
    CheckResult::from_utilization(clause_for_rule(rule), Quantity::force_kn(ed), Quantity::force_kn(resistance_kn), message_for_rule(rule, leading), annex.choice())
}

/// ✅️ Run all relevant combinations for an action set in a design situation.
pub fn check_combination_set<A: NationalAnnex>(annex: &A, situation: DesignSituation, actions: &ActionSet, resistance_kn: f64) -> CheckReport {
    let mut report = CheckReport::default();
    let n_leading = actions.q_k.len().max(1);
    for rule in rules_for_situation(situation, LimitState::Uls) {
        for leading in 0..n_leading {
            if actions.q_k.is_empty() && leading > 0 {
                break;
            }
            report.push(check_combination(annex, situation, rule, actions, leading, resistance_kn));
        }
    }
    for rule in rules_for_situation(situation, LimitState::Sls) {
        match rule {
            CombinationRule::SlsQuasiPermanent => {
                report.push(check_combination(annex, situation, rule, actions, 0, resistance_kn));
            }
            _ => {
                for leading in 0..n_leading {
                    if actions.q_k.is_empty() && leading > 0 {
                        break;
                    }
                    report.push(check_combination(annex, situation, rule, actions, leading, resistance_kn));
                }
            }
        }
    }
    report
}

/// ✅️ Check design action against resistance (ULS).
pub fn check_uls_action<A: NationalAnnex>(annex: &A, actions: &ActionSet, leading: usize, resistance: f64) -> CheckResult {
    let ed = combination_6_10(annex, actions, leading);
    CheckResult::from_utilization(ClauseId::new("EN 1990", "§6.4", "6.10"), Quantity::force_kn(ed), Quantity::force_kn(resistance), "ULS design action", annex.choice())
}
// #endregion 🔖️Combinations

// #region 🔖️Reliability
/// 📐️ Reliability index target β for RC2 (EN 1990 Annex C).
pub fn target_reliability_index(consequence_class: u8) -> f64 {
    match consequence_class {
        1 => 3.1,
        2 => 3.8,
        3 => 4.3,
        _ => 3.8,
    }
}

pub fn check_reliability_index(beta: f64, consequence_class: u8) -> CheckResult {
    let target = target_reliability_index(consequence_class);
    let passes = beta >= target;
    CheckResult {
        clause: ClauseId::new("EN 1990", "Annex C", "C.2"),
        status: if passes { CheckStatus::Pass } else { CheckStatus::Fail },
        computed: Quantity::new(crate::document::QuantityKind::Dimensionless, beta),
        limit: Quantity::new(crate::document::QuantityKind::Dimensionless, target),
        utilization: if passes { target / beta } else { beta / target },
        message: "reliability index β".into(),
        annex: AnnexChoice::En,
    }
}
// #endregion 🔖️Reliability

/// 🔁️ Append one design-situation's combination checks onto a shared report.
pub fn append_combination_set<A: NationalAnnex>(report: &mut CheckReport, annex: &A, situation: DesignSituation, actions: &ActionSet, resistance_kn: f64) {
    let sub = check_combination_set(annex, situation, actions, resistance_kn);
    report.checks.extend(sub.checks);
}

/// 📋️ Run EN 1990 design basis checks across persistent, accidental, and seismic situations.
pub fn check_design_basis<A: NationalAnnex>(annex: &A, actions: &ActionSet, resistance_kn: f64, consequence_class: u8) -> CheckReport {
    let mut report = CheckReport::default();
    append_combination_set(&mut report, annex, DesignSituation::Persistent, actions, resistance_kn);
    append_combination_set(&mut report, annex, DesignSituation::Accidental, actions, resistance_kn);
    append_combination_set(&mut report, annex, DesignSituation::Seismic, actions, resistance_kn);
    report.push(check_reliability_index(3.9, consequence_class));
    report
}

/// 🧮️ Seismic combination per EN 1990 Eq. 6.12b: ΣG_k + A_Ed + Σψ_2·Q_k.
pub fn combination_6_12b<A: NationalAnnex>(annex: &A, actions: &ActionSet, seismic_a_ed_kn: f64) -> f64 {
    let mut sum = actions.g_k + seismic_a_ed_kn;
    for (cat, q) in &actions.q_k {
        sum += annex.psi_2(cat) * q;
    }
    sum
}

/// ✅️ Check the seismic design situation per EN 1990 Eq. 6.12b.
pub fn check_seismic_situation<A: NationalAnnex>(annex: &A, actions: &ActionSet, seismic_a_ed_kn: f64, resistance_kn: f64) -> CheckResult {
    let ed = combination_6_12b(annex, actions, seismic_a_ed_kn);
    CheckResult::from_utilization(ClauseId::new("EN 1990", "§6.4.3.4", "6.12b"), Quantity::force_kn(ed), Quantity::force_kn(resistance_kn), "seismic design situation", annex.choice())
}
//#endregion 🔖️ComplianceHelpers

//#region 🧪️ComplianceHelpersTests
#[cfg(test)]
#[path = "🧪️tests/🔬️compliance-helpers/🦀️.rs"]
mod compliance_helpers_tests;
//#endregion 🧪️ComplianceHelpersTests
