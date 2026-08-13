//! 🧬️ En1990 artifact schema — every field of the artifact with its state class.

use schema::ArtifactSchema;
use crate::artifacts::en1990::En1990QkChild;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full En1990 artifact state across persistent and shared-ui classes. `q_k` mirrors
/// `En1990Snapshot`'s composed `s.stdio.semio.table` child slot (ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM round 2) — `to_snapshot`/`from_snapshot` copy the
/// handle across verbatim, same as `➗️mathematical`'s `MathematicalArtifact`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1990")]
pub struct En1990Artifact {
    #[state(persistent)] pub g_k: f64,
    #[state(persistent)]
    #[child(kind = "s.stdio.semio.table")]
    pub q_k: En1990QkChild,
    #[state(persistent)] pub resistance_kn: f64,
    #[state(persistent)] pub consequence_class: u8,
    #[state(persistent)] pub annex: crate::document::AnnexChoice,
    #[state(persistent)] pub seismic_a_ed_kn: f64,
    #[state(shared_ui)] pub selected_check_index: Option<u32>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl En1990Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::en1990::En1990Snapshot {
        crate::artifacts::en1990::En1990Snapshot {
            g_k: self.g_k,
            q_k: self.q_k.clone(),
            resistance_kn: self.resistance_kn,
            consequence_class: self.consequence_class,
            annex: self.annex,
            seismic_a_ed_kn: self.seismic_a_ed_kn,
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::en1990::En1990Snapshot) -> Self {
        Self {
            g_k: snapshot.g_k,
            q_k: snapshot.q_k.clone(),
            resistance_kn: snapshot.resistance_kn,
            consequence_class: snapshot.consequence_class,
            annex: snapshot.annex,
            seismic_a_ed_kn: snapshot.seismic_a_ed_kn,
            selected_check_index: None,
        }
    }
    /// 🔄 Overwrite persistent fields from a snapshot; leave shared-ui untouched.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::en1990::En1990Snapshot) {
        let selected = self.selected_check_index;
        *self = Self::from_snapshot(snapshot);
        self.selected_check_index = selected;
    }
}

//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.en1990` — twenty handcrafted schema leaves.
pub fn en1990_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.norm.en1990",
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
    use semio_framework_plugin::ArtifactBuilder;
    use crate::artifacts::en1990::{En1990Diff, En1990Mutation, En1990Snapshot};

    #[derive(Clone, Debug, Default)]
    pub struct En1990BuilderConstruction {
        snapshot: En1990Snapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for En1990BuilderConstruction {
        type Snapshot = En1990Snapshot;
        type Mutation = En1990Mutation;
        type Diff = En1990Diff;
        fn empty() -> Self { Self { snapshot: En1990Snapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<En1990Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<En1990Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let d = <En1990Mutation as protocol::Mutation<En1990Snapshot>>::diff(&mutation, &self.snapshot);
            self.snapshot = <En1990Diff as protocol::MutationDiff<En1990Snapshot>>::apply(&d, &self.snapshot);
            (self, d)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <En1990Diff as protocol::MutationDiff<En1990Snapshot>>::apply(&diff, &self.snapshot);
            self
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use semio_framework_plugin::{ArtifactAnalysis, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
    use crate::artifacts::en1990::En1990Snapshot;

    #[derive(Clone, Debug, Default)]
    pub struct En1990Parts {
        pub snapshot: Option<En1990Snapshot>,
    }

    pub struct En1990AnalyzerAnalysis;

    impl ArtifactAnalysis for En1990AnalyzerAnalysis {
        type Parts = En1990Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.en1990", standard: StandardId("1"), subset: SubsetId("*") };

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
        construction: derived_construction::En1990BuilderConstruction,
        analysis: derived_analysis::En1990AnalyzerAnalysis,
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
/// national-annex implementations are all pure over `ActionSet`/`&dyn NationalAnnex`, never over the
/// whole `En1990Snapshot`; the snapshot-level composition (`evaluate`) lives in `💡️inferences`.
/// `na_de`/`na_en` are depended on by several sibling EN 199x artifacts
/// (`crate::artifacts::en199x::standards::v1::subsets::any::schema::na_de::NaDe`).
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

pub fn psi_for_category(annex: &dyn NationalAnnex, category: &str) -> PsiRow {
    if annex.choice() == AnnexChoice::De {
        psi_row_de(category)
    } else {
        psi_row_en(category)
    }
}

pub fn psi_for_imposed(annex: &dyn NationalAnnex, category: ImposedCategory) -> PsiRow {
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

fn gamma_for_situation(annex: &dyn NationalAnnex, situation: DesignSituation) -> (f64, f64) {
    match situation {
        DesignSituation::Persistent | DesignSituation::Transient => (annex.gamma_g(), annex.gamma_q()),
        DesignSituation::Accidental | DesignSituation::Seismic => (1.0, 1.0),
    }
}

fn xi_for_situation(annex: &dyn NationalAnnex, situation: DesignSituation) -> f64 {
    match situation {
        DesignSituation::Persistent | DesignSituation::Transient => annex.xi("permanent"),
        DesignSituation::Accidental => annex.xi("accidental"),
        DesignSituation::Seismic => annex.xi("seismic"),
    }
}

/// 🧮️ ULS combination per EN 1990 Eq. 6.10: max(6.10a, 6.10b) surrogate as 6.10a.
pub fn combination_6_10(annex: &dyn NationalAnnex, actions: &ActionSet, leading: usize) -> f64 {
    combination_6_10a(annex, actions, leading)
}

/// 🧮️ ULS combination per EN 1990 Eq. 6.10a: γ_G·G + γ_Q·Q + γ_Q·ψ_0·ΣQ.
pub fn combination_6_10a(annex: &dyn NationalAnnex, actions: &ActionSet, leading: usize) -> f64 {
    let mut sum = annex.gamma_g() * actions.g_k;
    for (i, (cat, q)) in actions.q_k.iter().enumerate() {
        let factor = if i == leading { annex.gamma_q() } else { annex.gamma_q() * annex.psi_0(cat) };
        sum += factor * q;
    }
    sum
}

/// 🧮️ ULS combination per EN 1990 Eq. 6.10b: ξ·γ_G·G + γ_Q·Q + γ_Q·ψ_0·ΣQ.
pub fn combination_6_10b(annex: &dyn NationalAnnex, actions: &ActionSet, leading: usize) -> f64 {
    let xi = annex.xi("permanent");
    let mut sum = xi * annex.gamma_g() * actions.g_k;
    for (i, (cat, q)) in actions.q_k.iter().enumerate() {
        let factor = if i == leading { annex.gamma_q() } else { annex.gamma_q() * annex.psi_0(cat) };
        sum += factor * q;
    }
    sum
}

/// 🧮️ ULS combination for a design situation with situation-specific γ factors.
pub fn combination_uls(annex: &dyn NationalAnnex, situation: DesignSituation, rule: CombinationRule, actions: &ActionSet, leading: usize) -> f64 {
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
pub fn combination_sls_char(annex: &dyn NationalAnnex, actions: &ActionSet, leading: usize) -> f64 {
    let mut sum = actions.g_k;
    for (i, (cat, q)) in actions.q_k.iter().enumerate() {
        let factor = if i == leading { 1.0 } else { annex.psi_0(cat) };
        sum += factor * q;
    }
    sum
}

/// 🧮️ SLS frequent combination: G + ψ_1·Q_leading + ψ_2·ΣQ_accompanying.
pub fn combination_sls_frequent(annex: &dyn NationalAnnex, actions: &ActionSet, leading: usize) -> f64 {
    let mut sum = actions.g_k;
    for (i, (cat, q)) in actions.q_k.iter().enumerate() {
        let factor = if i == leading { annex.psi_1(cat) } else { annex.psi_2(cat) };
        sum += factor * q;
    }
    sum
}

/// 🧮️ SLS quasi-permanent combination: G + ψ_2·ΣQ.
pub fn combination_sls_quasi_permanent(annex: &dyn NationalAnnex, actions: &ActionSet) -> f64 {
    let mut sum = actions.g_k;
    for (cat, q) in &actions.q_k {
        sum += annex.psi_2(cat) * q;
    }
    sum
}

pub fn combination_value(annex: &dyn NationalAnnex, rule: CombinationRule, actions: &ActionSet, leading: usize) -> f64 {
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
pub fn check_combination(annex: &dyn NationalAnnex, situation: DesignSituation, rule: CombinationRule, actions: &ActionSet, leading: usize, resistance_kn: f64) -> CheckResult {
    let ed = if matches!(rule, CombinationRule::Uls610 | CombinationRule::Uls610a | CombinationRule::Uls610b) { combination_uls(annex, situation, rule, actions, leading) } else { combination_value(annex, rule, actions, leading) };
    CheckResult::from_utilization(clause_for_rule(rule), Quantity::force_kn(ed), Quantity::force_kn(resistance_kn), message_for_rule(rule, leading), annex.choice())
}

/// ✅️ Run all relevant combinations for an action set in a design situation.
pub fn check_combination_set(annex: &dyn NationalAnnex, situation: DesignSituation, actions: &ActionSet, resistance_kn: f64) -> CheckReport {
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
pub fn check_uls_action(annex: &dyn NationalAnnex, actions: &ActionSet, leading: usize, resistance: f64) -> CheckResult {
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
pub fn append_combination_set(report: &mut CheckReport, annex: &dyn NationalAnnex, situation: DesignSituation, actions: &ActionSet, resistance_kn: f64) {
    let sub = check_combination_set(annex, situation, actions, resistance_kn);
    report.checks.extend(sub.checks);
}

/// 📋️ Run EN 1990 design basis checks across persistent, accidental, and seismic situations.
pub fn check_design_basis(annex: &dyn NationalAnnex, actions: &ActionSet, resistance_kn: f64, consequence_class: u8) -> CheckReport {
    let mut report = CheckReport::default();
    append_combination_set(&mut report, annex, DesignSituation::Persistent, actions, resistance_kn);
    append_combination_set(&mut report, annex, DesignSituation::Accidental, actions, resistance_kn);
    append_combination_set(&mut report, annex, DesignSituation::Seismic, actions, resistance_kn);
    report.push(check_reliability_index(3.9, consequence_class));
    report
}

/// 🧮️ Seismic combination per EN 1990 Eq. 6.12b: ΣG_k + A_Ed + Σψ_2·Q_k.
pub fn combination_6_12b(annex: &dyn NationalAnnex, actions: &ActionSet, seismic_a_ed_kn: f64) -> f64 {
    let mut sum = actions.g_k + seismic_a_ed_kn;
    for (cat, q) in &actions.q_k {
        sum += annex.psi_2(cat) * q;
    }
    sum
}

/// ✅️ Check the seismic design situation per EN 1990 Eq. 6.12b.
pub fn check_seismic_situation(annex: &dyn NationalAnnex, actions: &ActionSet, seismic_a_ed_kn: f64, resistance_kn: f64) -> CheckResult {
    let ed = combination_6_12b(annex, actions, seismic_a_ed_kn);
    CheckResult::from_utilization(ClauseId::new("EN 1990", "§6.4.3.4", "6.12b"), Quantity::force_kn(ed), Quantity::force_kn(resistance_kn), "seismic design situation", annex.choice())
}
//#endregion 🔖️ComplianceHelpers

//#region 🧪️ComplianceHelpersTests
#[cfg(test)]
mod compliance_helpers_tests {
    use super::*;

    fn sample_actions() -> ActionSet {
        ActionSet { g_k: 100.0, q_k: vec![("office".into(), 50.0), ("wind".into(), 30.0)] }
    }

    #[test]
    fn de_na_combination_6_10() {
        let annex = NaDe;
        let actions = sample_actions();
        let ed = combination_6_10(&annex, &actions, 0);
        assert!(ed > 100.0);
        let report = check_design_basis(&annex, &actions, 300.0, 2);
        assert!(report.all_pass());
    }

    #[test]
    fn de_combination_6_10a_numeric() {
        let annex = NaDe;
        let actions = sample_actions();
        let ed = combination_6_10a(&annex, &actions, 0);
        assert!((ed - 237.0).abs() < 1e-9);
    }

    #[test]
    fn de_combination_6_10b_numeric() {
        let annex = NaDe;
        let actions = sample_actions();
        let ed = combination_6_10b(&annex, &actions, 0);
        assert!((ed - 216.75).abs() < 1e-9);
    }

    #[test]
    fn en_combination_6_10a_differs_on_other_psi() {
        let de = NaDe;
        let en = NaEn;
        let actions = ActionSet { g_k: 100.0, q_k: vec![("office".into(), 50.0), ("other".into(), 30.0)] };
        let de_ed = combination_6_10a(&de, &actions, 0);
        let en_ed = combination_6_10a(&en, &actions, 0);
        assert!((de_ed - 246.0).abs() < 1e-9);
        assert!((en_ed - 241.5).abs() < 1e-9);
        assert!(de_ed > en_ed);
    }

    #[test]
    fn de_vs_en_congregation_psi_tables() {
        let de = NaDe;
        let en = NaEn;
        assert!((de.psi_1("congregation") - 0.7).abs() < 1e-9);
        assert!((en.psi_1("congregation") - 0.7).abs() < 1e-9);
        assert!((de.psi_0("other") - 0.8).abs() < 1e-9);
        assert!((en.psi_0("other") - 0.7).abs() < 1e-9);
        assert!((de.psi_0("wind") - 0.6).abs() < 1e-9);
        assert!((en.psi_0("wind") - 0.6).abs() < 1e-9);
        let actions = ActionSet { g_k: 100.0, q_k: vec![("congregation".into(), 50.0)] };
        let de_freq = combination_sls_frequent(&de, &actions, 0);
        let en_freq = combination_sls_frequent(&en, &actions, 0);
        assert!((de_freq - 135.0).abs() < 1e-9);
        assert!((en_freq - 135.0).abs() < 1e-9);
        assert!((de.psi_2("storage") - 0.8).abs() < 1e-9);
        assert!((en.psi_2("storage") - 0.8).abs() < 1e-9);
        let qp_de = combination_sls_quasi_permanent(&de, &actions);
        let qp_en = combination_sls_quasi_permanent(&en, &actions);
        assert!((qp_de - 130.0).abs() < 1e-9);
        assert!((qp_en - 130.0).abs() < 1e-9);
    }

    #[test]
    fn de_na_gamma_m_and_xi() {
        let annex = NaDe;
        assert!((annex.gamma_m("concrete") - 1.5).abs() < 1e-9);
        assert!((annex.gamma_m("steel") - 1.0).abs() < 1e-9);
        assert!((annex.gamma_m("timber") - 1.3).abs() < 1e-9);
        assert!((annex.gamma_r() - 1.0).abs() < 1e-9);
        assert!((annex.xi("permanent") - 0.85).abs() < 1e-9);
    }

    #[test]
    fn imposed_categories_a_to_h_de() {
        let annex = NaDe;
        for cat in [ImposedCategory::A, ImposedCategory::B, ImposedCategory::C, ImposedCategory::D, ImposedCategory::E, ImposedCategory::F, ImposedCategory::G, ImposedCategory::H] {
            let row = psi_for_imposed(&annex, cat);
            let label = cat.label();
            assert!((annex.psi_0(label) - row.psi_0).abs() < 1e-9);
            assert!((annex.psi_1(label) - row.psi_1).abs() < 1e-9);
            assert!((annex.psi_2(label) - row.psi_2).abs() < 1e-9);
        }
        assert!((annex.psi_0("roof") - 0.0).abs() < 1e-9);
        assert!((annex.psi_0("storage") - 1.0).abs() < 1e-9);
    }

    #[test]
    fn check_combination_set_covers_uls_and_sls() {
        let annex = NaDe;
        let actions = sample_actions();
        let report = check_combination_set(&annex, DesignSituation::Persistent, &actions, 300.0);
        assert!(report.checks.len() >= 9);
        assert!(report.checks.iter().any(|c| c.clause.section == "6.10a"));
        assert!(report.checks.iter().any(|c| c.clause.section == "6.10b"));
        assert!(report.checks.iter().any(|c| c.clause.section == "6.16"));
        assert!(report.checks.iter().any(|c| c.clause.section == "6.17"));
    }

    #[test]
    fn accidental_situation_uses_unit_gamma() {
        let annex = NaDe;
        let actions = sample_actions();
        let persistent = combination_uls(&annex, DesignSituation::Persistent, CombinationRule::Uls610a, &actions, 0);
        let accidental = combination_uls(&annex, DesignSituation::Accidental, CombinationRule::Uls610a, &actions, 0);
        assert!(accidental < persistent);
        assert!((accidental - 168.0).abs() < 1e-9);
    }

    #[test]
    fn check_design_basis_covers_all_situations() {
        let annex = NaDe;
        let actions = sample_actions();
        let report = check_design_basis(&annex, &actions, 300.0, 2);
        let persistent = check_combination_set(&annex, DesignSituation::Persistent, &actions, 300.0);
        let accidental = check_combination_set(&annex, DesignSituation::Accidental, &actions, 300.0);
        let seismic = check_combination_set(&annex, DesignSituation::Seismic, &actions, 300.0);
        assert_eq!(report.checks.len(), persistent.checks.len() + accidental.checks.len() + seismic.checks.len() + 1);
    }

    #[test]
    fn seismic_combination_de_vs_en_diverge_on_other_psi_2() {
        let actions = ActionSet { g_k: 100.0, q_k: vec![("other".into(), 50.0)] };
        let de_ed = combination_6_12b(&NaDe, &actions, 40.0);
        let en_ed = combination_6_12b(&NaEn, &actions, 40.0);
        assert!((de_ed - 165.0).abs() < 1e-9);
        assert!((en_ed - 155.0).abs() < 1e-9);
        assert!(de_ed > en_ed);
    }
}
//#endregion 🧪️ComplianceHelpersTests
