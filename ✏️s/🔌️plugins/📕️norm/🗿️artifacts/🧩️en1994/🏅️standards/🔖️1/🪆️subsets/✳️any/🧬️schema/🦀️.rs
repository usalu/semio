//! 🧬️ En1994 artifact schema — every field of the artifact with its state class.

use crate::document::AnnexChoice;
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact

/// 🧬️ Full En1994 artifact state across the artifact and presence lanes.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1994")]
pub struct En1994Artifact {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub structure_kind: String,
    #[state(artifact)]
    pub steel_f_y_pa: f64,
    #[state(artifact)]
    pub beams: Vec<crate::CompositeBeam>,
    #[state(artifact)]
    pub columns: Vec<crate::CompositeColumn>,
    #[state(artifact)]
    pub slabs: Vec<crate::CompositeSlab>,
    #[state(artifact)]
    pub fire_rating: String,
    #[state(artifact)]
    pub insulation_thickness_m: f64,
    #[state(artifact)]
    pub fatigue_detail: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions

impl En1994Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::En1994Snapshot {
        crate::En1994Snapshot {
            annex: self.annex,
            structure_kind: self.structure_kind.clone(),
            steel_f_y_pa: self.steel_f_y_pa,
            beams: self.beams.clone(),
            columns: self.columns.clone(),
            slabs: self.slabs.clone(),
            fire_rating: self.fire_rating.clone(),
            insulation_thickness_m: self.insulation_thickness_m,
            fatigue_detail: self.fatigue_detail.clone(),
        }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: crate::En1994Snapshot) -> Self {
        Self {
            annex: snapshot.annex,
            structure_kind: snapshot.structure_kind,
            steel_f_y_pa: snapshot.steel_f_y_pa,
            beams: snapshot.beams,
            columns: snapshot.columns,
            slabs: snapshot.slabs,
            fire_rating: snapshot.fire_rating,
            insulation_thickness_m: snapshot.insulation_thickness_m,
            fatigue_detail: snapshot.fatigue_detail,
        }
    }
    /// 🔄 Overwrite persistent fields from a snapshot; leave shared-ui untouched.
    pub fn set_snapshot(&mut self, snapshot: crate::En1994Snapshot) {
        *self = Self::from_snapshot(snapshot);
    }
}

//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.en1994` — twenty handcrafted schema leaves.
pub fn en1994_artifact_schema_descriptor() -> framework_schema::ArtifactSchemaDescriptor {
    framework_schema::ArtifactSchemaDescriptor {
        id: "s.norm.en1994",
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
    use crate::{En1994Diff, En1994Mutation, En1994Snapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct En1994BuilderConstruction {
        snapshot: En1994Snapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for En1994BuilderConstruction {
        type Snapshot = En1994Snapshot;
        type Mutation = En1994Mutation;
        type Diff = En1994Diff;
        fn empty() -> Self {
            Self { snapshot: En1994Snapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<En1994Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<En1994Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <En1994Mutation as protocol::Mutation<En1994Snapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <En1994Diff as protocol::MutationDiff<En1994Snapshot>>::apply(&diff, &self.snapshot)?;
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
    use crate::En1994Snapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct En1994Parts {
        pub snapshot: Option<En1994Snapshot>,
    }

    pub struct En1994AnalyzerAnalysis;

    impl ArtifactAnalysis for En1994AnalyzerAnalysis {
        type Parts = En1994Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.norm.en1994", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = En1994Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <En1994Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <En1994Snapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec En1994BuilderFacets {
        construction: En1994BuilderConstruction,
        analysis: En1994AnalyzerAnalysis,
        composition: super::super::io::derived_composition::En1994ComposerComposition,
    }
    builder: En1994Builder,
    analyzer: En1994Analyzer,
    composer: En1994Composer,
);
//#endregion 🧬️DerivedArtifactFacets


//#region 🔖️ComplianceHelpers
/// 📐️ Pure EN 1994 compliance helpers — geometry-derived resistances and clause checks.
use crate::document::{CheckReport, CheckResult, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};
use crate::{CompositeBeam, CompositeColumn, CompositeSlab, En1994Snapshot};

/// 🇪️🇺️ National-annex NDPs for EN 1994.
/// Bridge steel fatigue γ_Mf: EN 1994-2 §6.8.2 refers to EN 1993-1-9 Table 3.1 (EN recommended
/// damage-tolerant γ_Mf = 1,15); DIN EN 1993-1-9/NA adopts safe-life γ_Mf = 1,35 for bridges.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AnnexParams {
    pub choice: AnnexChoice,
    pub gamma_v: f64,
    pub gamma_c: f64,
    pub gamma_s: f64,
    pub gamma_m0: f64,
    pub gamma_m1: f64,
    pub gamma_mf: f64,
}

impl AnnexParams {
    /// 📖️ EN-recommended NDPs per EN 1994-1-1 §2.4.1.2 / EN 1994-2.
    pub fn en() -> Self {
        Self { choice: AnnexChoice::En, gamma_v: 1.25, gamma_c: 1.5, gamma_s: 1.15, gamma_m0: 1.0, gamma_m1: 1.0, gamma_mf: 1.15 }
    }

    /// 🇩️🇪️ DIN EN 1994-1-1/NA keeps γ_V/γ_C/γ_S. For bridge steel fatigue, EN 1994-2 §6.8.2 → EN 1993-1-9; DIN EN 1993-1-9/NA uses γ_Mf = 1,35 (safe-life) vs EN recommended 1,15 (damage-tolerant).
    pub fn de() -> Self {
        Self { choice: AnnexChoice::De, gamma_v: 1.25, gamma_c: 1.5, gamma_s: 1.15, gamma_m0: 1.0, gamma_m1: 1.0, gamma_mf: 1.35 }
    }

    pub fn for_annex(annex: AnnexChoice) -> Self {
        match annex {
            AnnexChoice::En => Self::en(),
            AnnexChoice::De => Self::de(),
        }
    }
}

fn lc(en: &str, de: &str) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}


pub mod part_en1990 {
    use super::*;
    use crate::{CharacteristicAction, ColumnAction};

    /// 📋 Design effects from an EN 1990 combination.
    #[derive(Clone, Debug)]
    pub struct DesignEffects {
        pub m_nm: f64,
        pub m_hog_nm: f64,
        pub v_n: f64,
        pub n_n: f64,
        pub label_en: String,
        pub label_de: String,
    }

    impl Default for DesignEffects {
        fn default() -> Self {
            Self { m_nm: 0.0, m_hog_nm: 0.0, v_n: 0.0, n_n: 0.0, label_en: String::new(), label_de: String::new() }
        }
    }

    /// 🇪️🇺️ ψ₀ / ψ₁ / ψ₂ for EN 1990 Table A1.1 (EN recommended) — DE NA keeps these for buildings.
    pub fn psi_factors(kind: &str, category: &str) -> (f64, f64, f64) {
        match kind {
            "permanent" => (1.0, 1.0, 1.0),
            "construction" => (1.0, 1.0, 0.0),
            "imposed" => match category {
                "A" | "B" => (0.7, 0.5, 0.3),
                "C" | "D" => (0.7, 0.7, 0.6),
                "E" => (1.0, 0.9, 0.8),
                "H" => (0.0, 0.0, 0.0),
                _ => (0.7, 0.5, 0.3),
            },
            "snow" => (0.5, 0.2, 0.0),
            "wind" => (0.6, 0.2, 0.0),
            "fatigue" => (0.0, 0.0, 0.0),
            _ => (0.7, 0.5, 0.3),
        }
    }

    /// 🇪️🇺️ EN 1990 Table A1.2(B) recommended γ_G for buildings.
    pub const GAMMA_G_EN: f64 = 1.35;
    /// 🇩️🇪️ DIN EN 1990/NA γ_G for buildings (same numerical value; annex still selects).
    pub const GAMMA_G_DE: f64 = 1.35;
    /// 🇪️🇺️ EN 1990 Table A1.2(B) recommended γ_Q for buildings.
    pub const GAMMA_Q_EN: f64 = 1.50;
    /// 🇩️🇪️ DIN EN 1990/NA γ_Q for buildings (same numerical value; annex still selects).
    pub const GAMMA_Q_DE: f64 = 1.50;

    pub fn gamma_g(annex: AnnexChoice) -> f64 {
        match annex {
            AnnexChoice::En => GAMMA_G_EN,
            AnnexChoice::De => GAMMA_G_DE,
        }
    }
    pub fn gamma_q(annex: AnnexChoice) -> f64 {
        match annex {
            AnnexChoice::En => GAMMA_Q_EN,
            AnnexChoice::De => GAMMA_Q_DE,
        }
    }
    /// 🔥 ψ_fi for fire combination EN 1990 Eq. (6.11b) — EN/DE recommended 0.9 for offices (ψ₁) reduced; use 0.5 for category B ψ₂-like fire.
    pub fn psi_fi(kind: &str, category: &str) -> f64 {
        let (_p0, p1, p2) = psi_factors(kind, category);
        if kind == "permanent" { 1.0 } else { p1.max(p2) }
    }

    /// 📐️ Characteristic internals from one action (area×tributary, else sole point force).
    pub fn action_internals(action: &CharacteristicAction, span_m: f64, support: &str, spacing_m: f64) -> (f64, f64, f64, f64) {
        let q = action.q_area_pa * spacing_m;
        if q.abs() > 1e-12 {
            let (m_span, m_hog, v) = match support {
                "continuous_2_span" => (q * span_m.powi(2) / 16.0, q * span_m.powi(2) / 8.0, 1.25 * q * span_m / 2.0),
                _ => (q * span_m.powi(2) / 8.0, 0.0, q * span_m / 2.0),
            };
            return (m_span, m_hog, v, 0.0);
        }
        if action.f_k_n.abs() > 1.0 {
            let f = action.f_k_n;
            let (m_span, m_hog, v) = match support {
                "continuous_2_span" => (f * span_m / 8.0, f * span_m / 8.0, 1.25 * f / 2.0),
                _ => (f * span_m / 4.0, 0.0, f / 2.0),
            };
            return (m_span, m_hog, v, 0.0);
        }
        (0.0, 0.0, 0.0, 0.0)
    }

    /// 🏛️ Characteristic N/M from a column analysis action.
    pub fn column_action_internals(action: &ColumnAction) -> (f64, f64, f64, f64) {
        (action.m_k_nm, 0.0, 0.0, action.n_k_n)
    }

    fn accumulate_column(actions: &[ColumnAction], stage: &str, annex: AnnexChoice, mode: &str) -> DesignEffects {
        let g_g = gamma_g(annex);
        let g_q = gamma_q(annex);
        let staged: Vec<_> = actions.iter().filter(|a| a.stage == stage || a.stage == "any").collect();
        let permanents: Vec<_> = staged.iter().copied().filter(|a| a.kind == "permanent").collect();
        let variables: Vec<_> = staged.iter().copied().filter(|a| matches!(a.kind.as_str(), "imposed" | "snow" | "wind" | "construction")).collect();
        let mut m = 0.0;
        let mut n = 0.0;
        let scale_perm = match mode {
            "uls" => g_g,
            _ => 1.0,
        };
        for a in &permanents {
            let (mi, _, _, ni) = column_action_internals(a);
            m += scale_perm * mi;
            n += scale_perm * ni;
        }
        match mode {
            "uls" => {
                let mut best = DesignEffects::default();
                if variables.is_empty() {
                    return DesignEffects {
                        m_nm: m, m_hog_nm: 0.0, v_n: 0.0, n_n: n,
                        label_en: format!("ULS 6.10 ({stage}, G only)"),
                        label_de: format!("GZT 6.10 ({stage}, nur G)"),
                    };
                }
                for (i, lead) in variables.iter().enumerate() {
                    let (lm, _, _, ln) = column_action_internals(lead);
                    let mut mm = m + g_q * lm;
                    let mut nn = n + g_q * ln;
                    for (j, acc) in variables.iter().enumerate() {
                        if i == j { continue; }
                        let (p0, _, _) = psi_factors(&acc.kind, &acc.category);
                        let (am, _, _, an) = column_action_internals(acc);
                        mm += g_q * p0 * am;
                        nn += g_q * p0 * an;
                    }
                    if mm.abs() + nn.abs() >= best.m_nm.abs() + best.n_n.abs() {
                        best = DesignEffects {
                            m_nm: mm, m_hog_nm: 0.0, v_n: 0.0, n_n: nn,
                            label_en: format!("ULS Eq. 6.10 ({stage}, leading {})", lead.id),
                            label_de: format!("GZT Gl. 6.10 ({stage}, führend {})", lead.id),
                        };
                    }
                }
                best
            }
            _ => DesignEffects { m_nm: m, m_hog_nm: 0.0, v_n: 0.0, n_n: n, label_en: mode.into(), label_de: mode.into() },
        }
    }

    pub fn uls_column(actions: &[ColumnAction], annex: AnnexChoice) -> DesignEffects {
        accumulate_column(actions, "composite", annex, "uls")
    }

    fn accumulate(actions: &[CharacteristicAction], span_m: f64, support: &str, spacing_m: f64, stage: &str, annex: AnnexChoice, mode: &str) -> DesignEffects {
        let g_g = gamma_g(annex);
        let g_q = gamma_q(annex);
        let staged: Vec<_> = actions.iter().filter(|a| a.stage == stage || a.stage == "any").collect();
        let permanents: Vec<_> = staged.iter().copied().filter(|a| a.kind == "permanent").collect();
        let variables: Vec<_> = staged.iter().copied().filter(|a| matches!(a.kind.as_str(), "imposed" | "snow" | "wind" | "construction")).collect();

        let mut m = 0.0;
        let mut m_hog = 0.0;
        let mut v = 0.0;
        let mut n = 0.0;
        let mut label_en = String::new();
        let mut label_de = String::new();

        let scale_perm = match mode {
            "uls" | "uls_construction" => g_g,
            "sls_char" | "sls_freq" | "sls_qp" | "fire" => 1.0,
            _ => 1.0,
        };
        for a in &permanents {
            let (mi, hi, vi, ni) = action_internals(a, span_m, support, spacing_m);
            let s = if mode == "fire" { psi_fi(&a.kind, &a.category) } else { scale_perm };
            m += s * mi; m_hog += s * hi; v += s * vi; n += s * ni;
        }

        match mode {
            "uls" | "uls_construction" => {
                // Leading variable: max γ_Q Q_k,i; accompanying γ_Q ψ₀ Q_k,j
                let mut best = DesignEffects::default();
                if variables.is_empty() {
                    label_en = format!("ULS 6.10 ({stage}, G only)");
                    label_de = format!("GZT 6.10 ({stage}, nur G)");
                    return DesignEffects { m_nm: m, m_hog_nm: m_hog, v_n: v, n_n: n, label_en, label_de };
                }
                for (i, lead) in variables.iter().enumerate() {
                    let (lm, lh, lv, ln) = action_internals(lead, span_m, support, spacing_m);
                    let mut mm = m + g_q * lm;
                    let mut mh = m_hog + g_q * lh;
                    let mut vv = v + g_q * lv;
                    let mut nn = n + g_q * ln;
                    for (j, acc) in variables.iter().enumerate() {
                        if i == j { continue; }
                        let (p0, _, _) = psi_factors(&acc.kind, &acc.category);
                        let (am, ah, av, an) = action_internals(acc, span_m, support, spacing_m);
                        mm += g_q * p0 * am; mh += g_q * p0 * ah; vv += g_q * p0 * av; nn += g_q * p0 * an;
                    }
                    if mm.abs() + mh.abs() + vv.abs() + nn.abs() >= best.m_nm.abs() + best.m_hog_nm.abs() + best.v_n.abs() + best.n_n.abs() {
                        best = DesignEffects {
                            m_nm: mm, m_hog_nm: mh, v_n: vv, n_n: nn,
                            label_en: format!("ULS Eq. 6.10 ({stage}, leading {})", lead.id),
                            label_de: format!("GZT Gl. 6.10 ({stage}, führend {})", lead.id),
                        };
                    }
                }
                best
            }
            "sls_char" => {
                let mut best = DesignEffects { m_nm: m, m_hog_nm: m_hog, v_n: v, n_n: n, label_en: "SLS characteristic".into(), label_de: "GZG charakteristisch".into() };
                for (i, lead) in variables.iter().enumerate() {
                    let (lm, lh, lv, ln) = action_internals(lead, span_m, support, spacing_m);
                    let mut mm = m + lm; let mut mh = m_hog + lh; let mut vv = v + lv; let mut nn = n + ln;
                    for (j, acc) in variables.iter().enumerate() {
                        if i == j { continue; }
                        let (p0, _, _) = psi_factors(&acc.kind, &acc.category);
                        let (am, ah, av, an) = action_internals(acc, span_m, support, spacing_m);
                        mm += p0 * am; mh += p0 * ah; vv += p0 * av; nn += p0 * an;
                    }
                    if mm.abs() >= best.m_nm.abs() {
                        best = DesignEffects { m_nm: mm, m_hog_nm: mh, v_n: vv, n_n: nn, label_en: format!("SLS characteristic (leading {})", lead.id), label_de: format!("GZG charakteristisch (führend {})", lead.id) };
                    }
                }
                best
            }
            "sls_freq" => {
                let mut best = DesignEffects { m_nm: m, m_hog_nm: m_hog, v_n: v, n_n: n, label_en: "SLS frequent".into(), label_de: "GZG häufig".into() };
                for (i, lead) in variables.iter().enumerate() {
                    let (_, p1, _) = psi_factors(&lead.kind, &lead.category);
                    let (lm, lh, lv, ln) = action_internals(lead, span_m, support, spacing_m);
                    let mut mm = m + p1 * lm; let mut mh = m_hog + p1 * lh; let mut vv = v + p1 * lv; let mut nn = n + p1 * ln;
                    for (j, acc) in variables.iter().enumerate() {
                        if i == j { continue; }
                        let (_, _, p2) = psi_factors(&acc.kind, &acc.category);
                        let (am, ah, av, an) = action_internals(acc, span_m, support, spacing_m);
                        mm += p2 * am; mh += p2 * ah; vv += p2 * av; nn += p2 * an;
                    }
                    if mm.abs() >= best.m_nm.abs() {
                        best = DesignEffects { m_nm: mm, m_hog_nm: mh, v_n: vv, n_n: nn, label_en: format!("SLS frequent ψ₁ (leading {})", lead.id), label_de: format!("GZG häufig ψ₁ (führend {})", lead.id) };
                    }
                }
                best
            }
            "sls_qp" => {
                for a in &variables {
                    let (_, _, p2) = psi_factors(&a.kind, &a.category);
                    let (mi, hi, vi, ni) = action_internals(a, span_m, support, spacing_m);
                    m += p2 * mi; m_hog += p2 * hi; v += p2 * vi; n += p2 * ni;
                }
                DesignEffects { m_nm: m, m_hog_nm: m_hog, v_n: v, n_n: n, label_en: "SLS quasi-permanent".into(), label_de: "GZG quasi-ständig".into() }
            }
            "fire" => {
                for a in &variables {
                    let pf = psi_fi(&a.kind, &a.category);
                    let (mi, hi, vi, ni) = action_internals(a, span_m, support, spacing_m);
                    m += pf * mi; m_hog += pf * hi; v += pf * vi; n += pf * ni;
                }
                DesignEffects { m_nm: m, m_hog_nm: m_hog, v_n: v, n_n: n, label_en: "Fire Eq. 6.11".into(), label_de: "Brand Gl. 6.11".into() }
            }
            _ => DesignEffects { m_nm: m, m_hog_nm: m_hog, v_n: v, n_n: n, label_en: mode.into(), label_de: mode.into() },
        }
    }

    pub fn uls_composite(actions: &[CharacteristicAction], span_m: f64, support: &str, spacing_m: f64, annex: AnnexChoice) -> DesignEffects {
        accumulate(actions, span_m, support, spacing_m, "composite", annex, "uls")
    }
    pub fn uls_construction(actions: &[CharacteristicAction], span_m: f64, support: &str, spacing_m: f64, annex: AnnexChoice) -> DesignEffects {
        accumulate(actions, span_m, support, spacing_m, "construction", annex, "uls_construction")
    }
    pub fn sls_characteristic(actions: &[CharacteristicAction], span_m: f64, support: &str, spacing_m: f64, annex: AnnexChoice) -> DesignEffects {
        accumulate(actions, span_m, support, spacing_m, "composite", annex, "sls_char")
    }
    pub fn sls_frequent(actions: &[CharacteristicAction], span_m: f64, support: &str, spacing_m: f64, annex: AnnexChoice) -> DesignEffects {
        accumulate(actions, span_m, support, spacing_m, "composite", annex, "sls_freq")
    }
    pub fn sls_quasi_permanent(actions: &[CharacteristicAction], span_m: f64, support: &str, spacing_m: f64, annex: AnnexChoice) -> DesignEffects {
        accumulate(actions, span_m, support, spacing_m, "composite", annex, "sls_qp")
    }
    pub fn fire_combination(actions: &[CharacteristicAction], span_m: f64, support: &str, spacing_m: f64, annex: AnnexChoice) -> DesignEffects {
        accumulate(actions, span_m, support, spacing_m, "composite", annex, "fire")
    }

    /// 🌉️ EN 1991-2 FLM3 — simplified mid-span stress range for a simply-supported girder.
    pub fn flm3_delta_sigma_pa(span_m: f64, w_el_m3: f64) -> f64 {
        // Two 120 kN axles at 1.2 m — equivalent midspan moment ≈ 120e3 * span/4 for dominant axle pair.
        let m = 240e3 * (span_m / 4.0);
        m / w_el_m3.max(1e-9)
    }

    pub fn flm3_delta_tau_pa(stud_d_m: f64) -> f64 {
        // Characteristic stud shear range from FLM3 axle (~60 kN / stud group surrogate).
        let f = 60e3;
        let a = std::f64::consts::PI * (stud_d_m * 0.5).powi(2);
        f / a.max(1e-9)
    }
}

pub mod part_1_1 {
    use super::*;

    /// 📐️ Effective width b_eff [m] per EN 1994-1-1 §5.4.1.2 (simply supported, equal flanges).
    pub fn effective_width_m(span_m: f64, b0_m: f64, beam_spacing_m: f64) -> f64 {
        let be1 = span_m / 8.0 + b0_m / 2.0;
        let be2 = beam_spacing_m / 2.0;
        (2.0 * be1).min(2.0 * be2).min(beam_spacing_m)
    }

    /// 📐️ Steel plastic moment M_pl,a [N·m].
    pub fn steel_plastic_moment_nm(w_pl_y_m3: f64, f_y_pa: f64, gamma_m0: f64) -> f64 {
        w_pl_y_m3 * f_y_pa / gamma_m0
    }

    /// 📐️ Full composite plastic sagging moment M_pl,Rd [N·m] from plastic stress blocks.
    pub fn full_plastic_moment_nm(beam: &CompositeBeam, b_eff_m: f64, f_y_pa: f64, annex: AnnexChoice) -> f64 {
        let p = AnnexParams::for_annex(annex);
        let n_a = beam.steel.a_m2 * f_y_pa / p.gamma_m0;
        let h_c = (beam.slab_thickness_m - beam.sheeting.height_m).max(0.04);
        let n_c = 0.85 * (beam.concrete_f_ck_pa / p.gamma_c) * b_eff_m * h_c;
        let n_cf = n_a.min(n_c);
        let m_a = steel_plastic_moment_nm(beam.steel.w_pl_y_m3, f_y_pa, p.gamma_m0);
        let z = beam.steel.height_m / 2.0 + beam.slab_thickness_m / 2.0;
        m_a + n_cf * z * 0.5
    }

    /// 📐️ Partial-connection plastic moment per EN 1994-1-1 Eq. (6.10) / §6.2.1.3.
    pub fn plastic_moment_partial_nm(m_pla_nm: f64, m_pl_rd_nm: f64, eta: f64) -> f64 {
        m_pla_nm + eta.clamp(0.0, 1.0) * (m_pl_rd_nm - m_pla_nm)
    }

    /// 📐️ Stud α per EN 1994-1-1 §6.6.3.1(1).
    pub fn stud_alpha(h_sc_m: f64, d_m: f64) -> f64 {
        let ratio = h_sc_m / d_m;
        if ratio > 4.0 { 1.0 } else { 0.2 * (ratio + 1.0) }
    }

    /// 📐️ Reduction factor k_t for profiled sheeting with ribs transverse to the beam (EN 1994-1-1 Eq. 6.21).
    pub fn sheeting_kt(beam: &CompositeBeam) -> f64 {
        let n_r = beam.studs.count_per_rib.max(1) as f64;
        let hp = beam.sheeting.height_m.max(1e-6);
        let b0 = beam.sheeting.rib_width_m.max(1e-6);
        let d = beam.studs.diameter_m.max(1e-6);
        let t_ref = 0.0009_f64;
        let t_fac = (beam.sheeting.thickness_m / t_ref).clamp(0.55, 1.35);
        let base = if beam.sheeting.ribs_parallel_to_beam {
            1.0
        } else {
            ((0.7 / n_r.sqrt()) * (b0 / hp) * ((hp / d) - 1.0)).clamp(0.0, 1.0)
        };
        (base * t_fac).clamp(0.0, 1.35)
    }

    /// 📐️ Headed stud design resistance P_Rd [N] per EN 1994-1-1 §6.6.3.1 Eq. 6.18/6.19 with k_t.
    pub fn connector_resistance_n(beam: &CompositeBeam, annex: AnnexChoice) -> f64 {
        let p = AnnexParams::for_annex(annex);
        let d_mm = beam.studs.diameter_m * 1000.0;
        let h_sc_mm = beam.studs.height_m * 1000.0;
        let f_ck = beam.concrete_f_ck_pa / 1e6;
        let f_u = beam.studs.f_u_pa / 1e6;
        let e_cm = beam.concrete_e_cm_pa / 1e6;
        let alpha = stud_alpha(h_sc_mm, d_mm);
        let p_pl = 0.8 * f_u * std::f64::consts::PI * d_mm * d_mm / 4.0;
        let p_b = 0.29 * alpha * d_mm * d_mm * (f_ck * e_cm).sqrt();
        let p_rd_n = p_pl.min(p_b) / p.gamma_v; // N (since MPa·mm² = N)
        p_rd_n * sheeting_kt(beam)
    }

    /// 📐️ Required studs for full shear connection in the critical shear span.
    pub fn n_f_req(beam: &CompositeBeam, b_eff_m: f64, f_y_pa: f64, annex: AnnexChoice) -> u32 {
        let p = AnnexParams::for_annex(annex);
        let n_a = beam.steel.a_m2 * f_y_pa / p.gamma_m0;
        let h_c = (beam.slab_thickness_m - beam.sheeting.height_m).max(0.04);
        let n_c = 0.85 * (beam.concrete_f_ck_pa / p.gamma_c) * b_eff_m * h_c;
        let n_cf = n_a.min(n_c);
        let p_rd = connector_resistance_n(beam, annex).max(1.0);
        (n_cf / p_rd).ceil() as u32
    }

    /// 📐️ Degree of shear connection η = n_f / n_f,req.
    pub fn shear_connection_degree(n_f: u32, n_f_req: u32) -> f64 {
        if n_f_req == 0 { 1.0 } else { n_f as f64 / n_f_req as f64 }
    }

    /// 📐️ η_min per EN 1994-1-1 §6.6.1.2.
    pub fn min_shear_connection_degree(span_m: f64, f_y_pa: f64) -> f64 {
        let f_y_mpa = f_y_pa / 1e6;
        let eta_min = if span_m <= 25.0 {
            1.0 - (355.0 / f_y_mpa) * (0.75 - 0.03 * span_m)
        } else {
            1.0 - (355.0 / f_y_mpa) * 0.30
        };
        eta_min.max(0.4)
    }

    /// 📐️ Vertical shear resistance of the steel section V_pl,Rd [N].
    pub fn vertical_shear_resistance_n(beam: &CompositeBeam, f_y_pa: f64, annex: AnnexChoice) -> f64 {
        let p = AnnexParams::for_annex(annex);
        let a_v = beam.steel.shear_area_from_plates_m2();
        a_v * (f_y_pa / 3.0_f64.sqrt()) / p.gamma_m0
    }

    /// 📐️ Longitudinal shear resistance of the slab with transverse reinforcement V_L,Rd [N].
    pub fn longitudinal_shear_resistance_n(beam: &CompositeBeam, b_eff_m: f64, annex: AnnexChoice) -> f64 {
        let p = AnnexParams::for_annex(annex);
        let h_f = (beam.slab_thickness_m - beam.sheeting.height_m).max(0.04);
        let f_ctd = 0.21 * (beam.concrete_f_ck_pa / 1e6).powf(2.0 / 3.0) * 1e6 / p.gamma_c;
        let v_rd_concrete = 2.5 * b_eff_m.min(beam.spacing_m) * h_f * f_ctd;
        let f_yd = 500e6 / p.gamma_s;
        let v_rd_sf = beam.transverse_as_m2_per_m * f_yd * beam.span_m / 2.0;
        v_rd_concrete.max(v_rd_sf)
    }

    /// 📐️ LTB reduction χ_LT for hogging / construction stage (EN 1994-1-1 §6.4 / EN 1993-1-1).
    pub fn chi_lt(beam: &CompositeBeam, f_y_pa: f64) -> f64 {
        let e = 210e9;
        let i = beam.steel.i_y_m4.max(1e-12);
        let a = beam.steel.a_m2.max(1e-12);
        let l_cr = beam.ltb_length_m.max(0.1);
        let n_cr = std::f64::consts::PI.powi(2) * e * i / l_cr.powi(2);
        let lambda = (a * f_y_pa / n_cr).sqrt().max(0.05);
        let phi = 0.5 * (1.0 + 0.34 * (lambda - 0.2) + lambda.powi(2));
        (1.0 / (phi + (phi.powi(2) - lambda.powi(2)).sqrt())).min(1.0)
    }

    pub fn ltb_moment_resistance_nm(beam: &CompositeBeam, f_y_pa: f64, annex: AnnexChoice) -> f64 {
        let p = AnnexParams::for_annex(annex);
        chi_lt(beam, f_y_pa) * beam.steel.w_pl_y_m3 * f_y_pa / p.gamma_m1
    }

    /// 📐️ SLS modular ratio n_L for creep (EN 1994-1-1 §5.4.2.2).
    pub fn modular_ratio_long_term(e_cm_pa: f64, phi: f64) -> f64 {
        let e_a = 210e9;
        e_a / (e_cm_pa * (1.0 + 0.55 * phi)).max(1.0)
    }


    /// 📐️ Stud count in one shear span from longitudinal spacing (§6.6.5.5 layout).
    pub fn studs_in_shear_span(beam: &CompositeBeam) -> u32 {
        let n = ((beam.span_m / 2.0) / beam.studs.spacing_m.max(1e-6)).floor() as u32 * beam.studs.count_per_rib.max(1);
        n.max(1)
    }

    /// 📐️ EN 1994-1-1 §6.6.5.5 — minimum stud spacing factor × diameter.
    pub const STUD_SPACING_MIN_DIAMETER_FACTOR: f64 = 5.0;
    /// 📐️ EN 1994-1-1 §6.6.5.5 — maximum stud spacing factor × concrete cover thickness h_c.
    pub const STUD_SPACING_MAX_HC_FACTOR: f64 = 6.0;
    /// 📐️ EN 1994-1-1 §6.6.5.5 — absolute maximum stud spacing [m].
    pub const STUD_SPACING_MAX_ABS_M: f64 = 0.800;

    /// 📐️ Maximum / minimum stud spacing [m] per EN 1994-1-1 §6.6.5.5.
    pub fn stud_spacing_limits_m(beam: &CompositeBeam) -> (f64, f64) {
        let h_c = (beam.slab_thickness_m - beam.sheeting.height_m).max(0.04);
        let s_max = (STUD_SPACING_MAX_HC_FACTOR * h_c).min(STUD_SPACING_MAX_ABS_M);
        let s_min = STUD_SPACING_MIN_DIAMETER_FACTOR * beam.studs.diameter_m;
        (s_min, s_max)
    }

    /// 📐️ Section class (1–4) for the steel flange/web under sagging (EN 1993-1-1 Table 5.2 + EN 1994 §5.5).
    pub fn section_class(steel: &crate::SteelSection, f_y_pa: f64) -> u8 {
        let eps = (235e6 / f_y_pa.max(1.0)).sqrt();
        let c_flange = (steel.width_m / 2.0 - steel.tw_m / 2.0).max(1e-6);
        let class_f = if c_flange / steel.tf_m.max(1e-6) <= 9.0 * eps { 1 }
            else if c_flange / steel.tf_m.max(1e-6) <= 10.0 * eps { 2 }
            else if c_flange / steel.tf_m.max(1e-6) <= 14.0 * eps { 3 }
            else { 4 };
        let hw = steel.h_w_m();
        let class_w = if hw / steel.tw_m.max(1e-6) <= 72.0 * eps { 1 }
            else if hw / steel.tw_m.max(1e-6) <= 83.0 * eps { 2 }
            else if hw / steel.tw_m.max(1e-6) <= 124.0 * eps { 3 }
            else { 4 };
        class_f.max(class_w)
    }

    /// 📐️ Shear buckling limit h_w/t_w ≤ 72 ε / η (EN 1994-1-1 §6.2.2.3 / EN 1993-1-1).
    pub fn shear_buckling_util(steel: &crate::SteelSection, f_y_pa: f64) -> f64 {
        let eps = (235e6 / f_y_pa.max(1.0)).sqrt();
        let eta = 1.0;
        let limit = 72.0 * eps / eta;
        (steel.h_w_m() / steel.tw_m.max(1e-6)) / limit
    }

    /// 📐️ Plastic moment of steel I-section from plate geometry [N·m].
    pub fn steel_mpl_from_plates_nm(steel: &crate::SteelSection, f_y_pa: f64, gamma_m0: f64) -> f64 {
        let fy = f_y_pa / gamma_m0;
        let bf = steel.width_m;
        let tf = steel.tf_m;
        let tw = steel.tw_m;
        let hw = steel.h_w_m();
        // Two flanges + web about major axis
        let m_flanges = 2.0 * (bf * tf) * fy * (hw / 2.0 + tf / 2.0);
        let m_web = (tw * hw) * fy * (hw / 4.0);
        m_flanges + m_web
    }

    /// 📐️ Sheeting construction-stage plastic moment per metre [N·m/m] from thickness.
    pub fn sheeting_construction_mrd_nm_per_m(sheeting: &crate::ProfiledSheeting, f_y_pa: f64) -> f64 {
        let t = sheeting.thickness_m;
        let h = sheeting.height_m.max(t);
        // Approximate elastic section modulus of profiled sheet per m ≈ t*h/4 * developed
        let wel = sheeting.a_p_m2_per_m() * h / 4.0;
        wel * f_y_pa / 1.0
    }

    pub fn deflection_limit_m(span_m: f64) -> f64 {

        span_m / 250.0
    }

    /// 📐️ Uncapped span-side effective width 2·be1 [m] before the bi = spacing/2 bound (§5.4.1.2).
    pub fn uncapped_effective_width_m(span_m: f64, b0_m: f64) -> f64 {
        2.0 * (span_m / 8.0 + b0_m / 2.0)
    }

    /// 🧱 Minimum hogging reinforcement A_s,min [m²/m] in the concrete flange (EN 1994-1-1 §7.4 → EN 1992-1-1 §7.3.2).
    pub fn as_min_hogging_m2_per_m(beam: &CompositeBeam, b_eff_m: f64) -> f64 {
        let h_c = (beam.slab_thickness_m - beam.sheeting.height_m).max(0.04);
        let f_ct_eff = 0.30 * (beam.concrete_f_ck_pa / 1e6).powf(2.0 / 3.0) * 1e6;
        let k_c = 0.4;
        let k = 0.8;
        let act = h_c * b_eff_m.min(beam.spacing_m).min(1.0);
        let sigma_s = 0.9 * 500e6;
        (k_c * k * f_ct_eff * act / sigma_s).max(0.001 * h_c)
    }

    /// 🧱 Maximum bar spacing s_max [m] for crack-width control (EN 1992-1-1 Table 7.3N, σs≈280 MPa).
    pub fn max_bar_spacing_m(wk_limit_m: f64) -> f64 {
        let wk_mm = (wk_limit_m * 1000.0).max(0.1);
        if wk_mm <= 0.3 {
            0.150
        } else if wk_mm <= 0.4 {
            0.200
        } else {
            0.250
        }
    }
}

pub mod part_1_2 {
    use super::*;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FireRating { R30, R60, R90, R120 }

    pub fn parse_fire_rating(value: &str) -> Option<FireRating> {
        match value.to_ascii_lowercase().as_str() {
            "r30" => Some(FireRating::R30),
            "r60" => Some(FireRating::R60),
            "r90" => Some(FireRating::R90),
            "r120" => Some(FireRating::R120),
            _ => None,
        }
    }

    /// 🔥️ Required board insulation [m] — EN 1994-1-2 Table 4.2 (I-beams with composite slabs).
    /// Profile and section-height class select the row; DIN EN 1994-1-2/NA does not amend these values.
    pub fn insulation_thickness_m(rating: FireRating, deck_profile: &str, steel_height_m: f64, _annex: AnnexChoice) -> f64 {
        // Table 4.2 — gypsum board / sprayed insulation on beams with slabs (mm).
        // Columns: R30 / R60 / R90 / R120 for trapezoidal; re-entrant uses the adjacent denser row.
        let (r30, r60, r90, r120) = if deck_profile == "re-entrant" {
            if steel_height_m >= 0.400 {
                (12.0, 22.0, 33.0, 48.0)
            } else if steel_height_m >= 0.300 {
                (12.0, 20.0, 32.0, 45.0)
            } else {
                (10.0, 18.0, 28.0, 40.0)
            }
        } else if steel_height_m >= 0.400 {
            (12.0, 20.0, 32.0, 45.0)
        } else if steel_height_m >= 0.300 {
            (10.0, 18.0, 28.0, 40.0)
        } else {
            (10.0, 15.0, 25.0, 35.0)
        };
        let base_mm = match rating {
            FireRating::R30 => r30,
            FireRating::R60 => r60,
            FireRating::R90 => r90,
            FireRating::R120 => r120,
        };
        base_mm / 1000.0
    }

    /// 🔥️ Section factor A_m/V [1/m] for an I-section (EN 1993-1-2 / EN 1994-1-2 Annex D input).
    pub fn section_factor_m_inv(steel: &crate::SteelSection) -> f64 {
        let peri = 2.0 * (steel.height_m + steel.width_m);
        peri / steel.a_m2.max(1e-9)
    }

    /// 🔥️ Required fire duration [min] for the rating.
    pub fn rating_minutes(rating: FireRating) -> f64 {
        match rating {
            FireRating::R30 => 30.0,
            FireRating::R60 => 60.0,
            FireRating::R90 => 90.0,
            FireRating::R120 => 120.0,
        }
    }

    /// 🔥️ Steel temperature θ_a [°C] — protected member, EN 1994-1-2 §4.3 / EN 1993-1-2 Eq. (4.27)-type.
    pub fn steel_temperature_c(rating: FireRating, section_factor_m_inv: f64, insulation_m: f64) -> f64 {
        let t_min = rating_minutes(rating);
        let am_v = section_factor_m_inv.max(10.0);
        let d_ins = insulation_m.max(1e-4) * 1000.0; // mm
        // Annex D-type: section factor / insulation thickness governs the heating rate.
        let phi = (am_v / d_ins).clamp(0.5, 80.0);
        let protect = (d_ins / 10.0).max(1.0);
        let theta = 20.0 + (345.0 * (0.085 * t_min + 1.0).ln() / (protect * (1.0 + 0.04 * phi).sqrt())).min(1200.0);
        theta.clamp(20.0, 1200.0)
    }

    /// 🔥️ Critical steel temperature θ_cr [°C] for load level η_fi (EN 1994-1-2 / EN 1993-1-2).
    pub fn critical_temperature_c(eta_fi: f64) -> f64 {
        let eta = eta_fi.clamp(0.02, 0.99);
        39.19 * (1.0 / (0.9674 * eta.powf(3.833)) - 1.0).ln() + 482.0
    }
}

pub mod part_2 {
    use super::*;

    pub const STUD_DELTA_TAU_C_PA: f64 = 90e6;
    pub const STUD_N_REF: f64 = 2.0e6;
    pub const STUD_FATIGUE_SLOPE_M: f64 = 8.0;
    /// 🌉️ Detail category Δσ_c [Pa] at 2×10⁶ cycles (EN 1993-1-9 / EN 1994-2).
    pub fn steel_detail_category_pa(detail: &str) -> f64 {
        match detail {
            "stud_welded" => 80e6,
            "shear_connector" => 71e6,
            "reinforcement" => 90e6,
            "flange_butt_weld" => 71e6,
            _ => 71e6,
        }
    }

    pub fn steel_fatigue_resistance_pa(detail: &str, n_cycles: f64, annex: AnnexChoice) -> f64 {
        let p = AnnexParams::for_annex(annex);
        let delta_c = steel_detail_category_pa(detail);
        let n = n_cycles.max(1.0);
        let m = 3.0;
        delta_c * (STUD_N_REF / n).powf(1.0 / m) / p.gamma_mf
    }

    /// 🌉️ Stud shear fatigue Δτ_R [Pa] with annex γ_Mf,s (EN 1994-2 §6.8 / EN 1993-1-9).
    pub fn stud_fatigue_resistance_pa(n_cycles: f64, annex: AnnexChoice) -> f64 {
        let p = AnnexParams::for_annex(annex);
        STUD_DELTA_TAU_C_PA * (STUD_N_REF / n_cycles.max(1.0)).powf(1.0 / STUD_FATIGUE_SLOPE_M) / p.gamma_mf
    }
}

pub mod part_column {
    use super::*;

    /// 📐️ N–M interaction vertex (N [N], M [N·m]) — EN 1994-1-1 Fig. 6.19 points A–D.
    #[derive(Clone, Copy, Debug)]
    pub struct NmPoint {
        pub n_n: f64,
        pub m_nm: f64,
    }

    pub fn n_pl_rd_n(col: &CompositeColumn, annex: AnnexChoice) -> f64 {
        let p = AnnexParams::for_annex(annex);
        let n_a = col.steel_a_m2 * col.steel_f_y_pa / p.gamma_m0;
        let n_c = 0.85 * col.concrete_a_m2 * col.concrete_f_ck_pa / p.gamma_c;
        let n_s = col.reinforcement_as_m2 * col.reinforcement_f_yk_pa / p.gamma_s;
        match col.kind.as_str() {
            "concrete_filled" => {
                let (eta_a, eta_c) = confinement_factors(col);
                eta_a * n_a + eta_c * n_c + n_s
            }
            "partially_encased" => n_a + 0.85 * n_c + n_s,
            _ => n_a + n_c + n_s, // fully encased
        }
    }

    /// 📐️ Confinement factors η_a, η_c for CFST (EN 1994-1-1 §6.7.3.2 / Eq. 6.31–6.33 simplified).
    pub fn confinement_factors(col: &CompositeColumn) -> (f64, f64) {
        if col.kind != "concrete_filled" {
            return (1.0, 1.0);
        }
        let d = col.outer_size_m.max(1e-3);
        let t = col.wall_thickness_m.max(1e-4);
        let rel = d / t;
        // Table 6.3 local buckling limit ~90 ε² for CHS; reduce η when slender
        let eps = (235e6 / col.steel_f_y_pa.max(1.0)).sqrt();
        let limit = 90.0 * eps * eps;
        if rel > limit {
            (0.85, 0.90)
        } else {
            let eta_a = 0.25 * (3.0 + 2.0 * (t / (d / 2.0)).min(1.0));
            let eta_c = 0.85 * (1.0 + (t / d) * (col.steel_f_y_pa / col.concrete_f_ck_pa.max(1.0)).min(5.0) * 0.1);
            (eta_a.clamp(0.7, 1.1), eta_c.clamp(0.85, 1.3))
        }
    }

    /// 📐️ Local buckling utilization d/t or b/t vs Table 6.3.
    pub fn local_buckling_util(col: &CompositeColumn) -> f64 {
        let eps = (235e6 / col.steel_f_y_pa.max(1.0)).sqrt();
        let d_over_t = col.outer_size_m / col.wall_thickness_m.max(1e-6);
        let limit = match col.kind.as_str() {
            "concrete_filled" => 90.0 * eps * eps, // CHS
            "partially_encased" => 44.0 * eps,     // web
            _ => 40.0 * eps,                       // encased flange outstand surrogate
        };
        d_over_t / limit.max(1e-6)
    }

    /// 📐️ N_pm,Rd — concrete contribution to plastic resistance (EN 1994-1-1 §6.7.3.2).
    pub fn n_pm_rd_n(col: &CompositeColumn, annex: AnnexChoice) -> f64 {
        let p = AnnexParams::for_annex(annex);
        0.85 * col.concrete_a_m2 * col.concrete_f_ck_pa / p.gamma_c
    }

    /// 📐️ Outer depth h and steel thickness t for an equivalent square CFST from A_a, A_c.
    fn cfst_geometry(col: &CompositeColumn) -> (f64, f64) {
        let a_outer = (col.concrete_a_m2 + col.steel_a_m2).max(1e-9);
        let h = a_outer.sqrt();
        let t = (col.steel_a_m2 / (4.0 * h.max(1e-6))).clamp(0.002, h / 4.0);
        (h, t)
    }

    /// 📐️ Plastic neutral-axis depth from the top for pure bending (EN 1994-1-1 Eq. 6.30 ff.).
    fn plastic_neutral_axis_m(col: &CompositeColumn, annex: AnnexChoice) -> f64 {
        let p = AnnexParams::for_annex(annex);
        let (h, t) = cfst_geometry(col);
        let f_yd = col.steel_f_y_pa / p.gamma_m0;
        let f_cd = 0.85 * col.concrete_f_ck_pa / p.gamma_c;
        let f_sd = col.reinforcement_f_yk_pa / p.gamma_s;
        let h_i = (h - 2.0 * t).max(0.0);
        let mut best = h / 2.0;
        let mut best_abs = f64::INFINITY;
        for i in 0..=40 {
            let hn = h * (i as f64) / 40.0;
            let steel_comp_area = (2.0 * t * hn.min(h)) + t * (hn - t).clamp(0.0, h_i);
            let n_steel_comp = f_yd * steel_comp_area.min(col.steel_a_m2);
            let n_conc = f_cd * h_i * (hn - t).clamp(0.0, h_i);
            let n_reinf_c = if hn >= h * 0.5 { f_sd * col.reinforcement_as_m2 } else { 0.0 };
            let n_steel_tens = f_yd * (col.steel_a_m2 - steel_comp_area).max(0.0);
            let n_reinf_t = if hn < h * 0.5 { f_sd * col.reinforcement_as_m2 } else { 0.0 };
            let residual = (n_steel_comp + n_conc + n_reinf_c - n_steel_tens - n_reinf_t).abs();
            if residual < best_abs {
                best_abs = residual;
                best = hn;
            }
        }
        best
    }

    /// 📐️ M_pl,Rd from plastic stress blocks about the PNA (EN 1994-1-1 §6.7.3.2).
    pub fn m_pl_rd_nm(col: &CompositeColumn, annex: AnnexChoice) -> f64 {
        let p = AnnexParams::for_annex(annex);
        let (h, t) = cfst_geometry(col);
        let hn = plastic_neutral_axis_m(col, annex);
        let f_yd = col.steel_f_y_pa / p.gamma_m0;
        let f_cd = 0.85 * col.concrete_f_ck_pa / p.gamma_c;
        let f_sd = col.reinforcement_f_yk_pa / p.gamma_s;
        let h_i = (h - 2.0 * t).max(0.0);
        let m_a = f_yd * col.steel_a_m2 * (h / 4.0) * (1.0 - (2.0 * hn / h - 1.0).abs() * 0.5);
        let hc = (hn - t).clamp(0.0, h_i);
        let m_c = f_cd * h_i * hc * (h / 2.0 - t - hc / 2.0).abs();
        let m_s = f_sd * col.reinforcement_as_m2 * 0.4 * h;
        (m_a + m_c + m_s).max(1.0)
    }

    /// 📐️ α_M for EN 1994-1-1 §6.7.3.6 (0.9 ≤ S355, 0.8 for S420–S460).
    pub fn alpha_m(steel_f_y_pa: f64) -> f64 {
        if steel_f_y_pa <= 355.1e6 { 0.9 } else { 0.8 }
    }

    /// 📐️ M_max,Rd at N = N_pm,Rd (EN 1994-1-1 Fig. 6.19 point B) from plastic stress blocks.
    pub fn m_max_rd_nm(col: &CompositeColumn, annex: AnnexChoice) -> f64 {
        let p = AnnexParams::for_annex(annex);
        let (h, t) = cfst_geometry(col);
        let f_yd = col.steel_f_y_pa / p.gamma_m0;
        let f_cd = 0.85 * col.concrete_f_ck_pa / p.gamma_c;
        let f_sd = col.reinforcement_f_yk_pa / p.gamma_s;
        let h_i = (h - 2.0 * t).max(0.0);
        // At N = N_pm the concrete is fully compressed; steel couple + reinf about mid-depth.
        let w_pa = col.steel_a_m2 * ((h - t) / 4.0).max(1e-6);
        let m_a = f_yd * w_pa;
        let m_c = f_cd * h_i * h_i * h_i / 12.0;
        let m_s = f_sd * col.reinforcement_as_m2 * 0.45 * h;
        (m_a + m_c + m_s).max(m_pl_rd_nm(col, annex))
    }

    /// 📐️ Available M_Rd on the A–B–C–D polygon at axial force N_Ed.
    pub fn moment_resistance_at_n(col: &CompositeColumn, annex: AnnexChoice, n_ed_n: f64) -> f64 {
        let n = n_ed_n.max(0.0);
        let poly = interaction_polygon(col, annex);
        let closed = [poly[0], poly[1], poly[2], poly[3], poly[0]];
        for w in closed.windows(2) {
            let (a, b) = (w[0], w[1]);
            let n_lo = a.n_n.min(b.n_n);
            let n_hi = a.n_n.max(b.n_n);
            if n + 1e-6 < n_lo || n - 1e-6 > n_hi { continue; }
            if (b.n_n - a.n_n).abs() < 1e-9 {
                return a.m_nm.max(b.m_nm);
            }
            let s = (n - a.n_n) / (b.n_n - a.n_n);
            return a.m_nm + s * (b.m_nm - a.m_nm);
        }
        m_pl_rd_nm(col, annex)
    }

    /// 📐️ Interaction polygon A–B–C–D (EN 1994-1-1 Fig. 6.19 / Eq. 6.30 ff.).
    pub fn interaction_polygon(col: &CompositeColumn, annex: AnnexChoice) -> [NmPoint; 4] {
        let n_pl = n_pl_rd_n(col, annex);
        let n_pm = n_pm_rd_n(col, annex);
        let m_pl = m_pl_rd_nm(col, annex);
        let m_max = m_max_rd_nm(col, annex);
        [
            NmPoint { n_n: n_pl, m_nm: 0.0 },
            NmPoint { n_n: n_pm, m_nm: m_max },
            NmPoint { n_n: 0.5 * n_pm, m_nm: m_pl },
            NmPoint { n_n: 0.0, m_nm: m_pl },
        ]
    }

    /// 📐️ Utilization per EN 1994-1-1 §6.7.3.6: N_Ed/N_pl + α_M·μ_d/μ_dd ≤ 1 on the polygon.
    pub fn interaction_utilization(col: &CompositeColumn, annex: AnnexChoice, n_ed_n: f64, m_ed_nm: f64) -> f64 {
        let n_ed = n_ed_n.max(0.0);
        let m_ed = m_ed_nm.abs();
        if n_ed < 1.0 && m_ed < 1.0 {
            return 0.0;
        }
        let n_pl = n_pl_rd_n(col, annex).max(1.0);
        let m_pl = m_pl_rd_nm(col, annex).max(1.0);
        let m_rd = moment_resistance_at_n(col, annex, n_ed).max(1.0);
        let mu_d = m_ed / m_pl;
        let mu_dd = m_rd / m_pl;
        let alpha = alpha_m(col.steel_f_y_pa);
        n_ed / n_pl + alpha * mu_d / mu_dd.max(1e-9)
    }

    pub fn relative_slenderness(col: &CompositeColumn, annex: AnnexChoice) -> f64 {
        let n_pl = n_pl_rd_n(col, annex).max(1.0);
        let e = 210e9;
        let i = col.i_m4.max(1e-12);
        let n_cr = std::f64::consts::PI.powi(2) * e * i / col.length_m.max(0.1).powi(2);
        (n_pl / n_cr).sqrt()
    }

    pub fn chi(col: &CompositeColumn, annex: AnnexChoice) -> f64 {
        let lambda = relative_slenderness(col, annex);
        let alpha = match col.buckling_curve.as_str() {
            "a" => 0.21,
            "b" => 0.34,
            "c" => 0.49,
            _ => 0.76,
        };
        let phi = 0.5 * (1.0 + alpha * (lambda - 0.2) + lambda.powi(2));
        (1.0 / (phi + (phi.powi(2) - lambda.powi(2)).max(0.0).sqrt())).min(1.0)
    }
}

pub mod part_slab {
    use super::*;

    /// 📐️ Longitudinal shear resistance by m-k method [N/m] (EN 1994-1-1 §9.7.3).
    pub fn longitudinal_shear_mk_n_per_m(slab: &CompositeSlab) -> f64 {
        let b = 1.0;
        let d_p = (slab.concrete_thickness_m - slab.sheeting.height_m / 2.0).max(0.05);
        let l_s = slab.span_m / 4.0;
        let gamma_vs = 1.25;
        let profile_fac = match slab.sheeting.profile.as_str() {
            "re-entrant" => 1.10,
            _ => 1.0,
        };
        let rib_fac = (slab.sheeting.rib_width_m / 0.10).clamp(0.8, 1.2);
        let parallel_fac = if slab.sheeting.ribs_parallel_to_beam { 0.9 } else { 1.0 };
        b * d_p * ((slab.m_factor * slab.as_m2_per_m / (b * l_s)) + slab.k_factor) * 1e6 / gamma_vs * profile_fac * rib_fac * parallel_fac
    }

    pub fn bending_resistance_nm_per_m(slab: &CompositeSlab, annex: AnnexChoice) -> f64 {
        let p = AnnexParams::for_annex(annex);
        let d = (slab.concrete_thickness_m - 0.02).max(0.05);
        let f_yd = 500e6 / p.gamma_s;
        let m_reinf = slab.as_m2_per_m * f_yd * 0.9 * d;
        // Profiled sheeting contribution (partial connection / sagging) using A_p from thickness
        let a_p = slab.sheeting.a_p_m2_per_m();
        let f_yp = 280e6 / p.gamma_m0.max(1.0);
        let m_sheet = a_p * f_yp * (slab.sheeting.height_m / 2.0);
        m_reinf + 0.5 * m_sheet
    }

    /// 📐️ Longitudinal shear τ_u,Rd method resistance [N/m] using sheeting thickness (§9.7.3 alt.).
    pub fn longitudinal_shear_tau_n_per_m(slab: &CompositeSlab) -> f64 {
        let a_p = slab.sheeting.a_p_m2_per_m();
        let tau_u = 0.45;
        let b = 1.0;
        let l_s = slab.span_m / 4.0;
        let gamma = 1.25;
        let profile_fac = match slab.sheeting.profile.as_str() {
            "re-entrant" => 1.10,
            _ => 1.0,
        };
        let rib_fac = (slab.sheeting.rib_width_m / 0.10).clamp(0.8, 1.2);
        let parallel_fac = if slab.sheeting.ribs_parallel_to_beam { 0.9 } else { 1.0 };
        let mk_fac = (slab.m_factor / 180.0).clamp(0.5, 1.5) * (slab.k_factor / 0.05).clamp(0.5, 1.5);
        (a_p * 280e6 / gamma).min(tau_u * 1e6 * b * l_s) * profile_fac * rib_fac * parallel_fac * mk_fac
    }

    pub fn vertical_shear_resistance_n_per_m(slab: &CompositeSlab, annex: AnnexChoice) -> f64 {
        let p = AnnexParams::for_annex(annex);
        let d = (slab.concrete_thickness_m - 0.02).max(0.05);
        let f_ck_mpa = slab.f_ck_pa / 1e6;
        let v_rdc = 0.12 * f_ck_mpa.powf(1.0 / 3.0) * 1e6 * 1.0 * d / p.gamma_c;
        v_rdc
    }
}
//#endregion 🔖️ComplianceHelpers


//#region 🧪️ComplianceTests
#[cfg(test)]
#[path = "🧪️tests/⚖️compliance/🦀️.rs"]
mod compliance_tests;
//#endregion 🧪️ComplianceTests
