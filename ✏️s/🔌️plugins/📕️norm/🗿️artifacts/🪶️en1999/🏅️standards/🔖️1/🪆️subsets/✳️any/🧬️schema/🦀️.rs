//! ✨️ EN 1999 artifact schema — aluminium structure subject + DIN EN 1999 clause checks.

use crate::document::{
    AnnexChoice, CheckReport, CheckResult, CheckStatus, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef,
};
use crate::snapshot::{ColdFormedSheet, AluminiumShell, AluminiumConnection, AluminiumMaterial, AluminiumMember, AluminiumSection, FatigueDetail, FireScenario};
use crate::En1999Snapshot;
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ EN 1999 document artifact state (mirrors snapshot subject).
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1999")]
pub struct En1999Artifact {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub materials: Vec<AluminiumMaterial>,
    #[state(artifact)]
    pub sections: Vec<AluminiumSection>,
    #[state(artifact)]
    pub members: Vec<AluminiumMember>,
    #[state(artifact)]
    pub connections: Vec<AluminiumConnection>,
    #[state(artifact)]
    pub fire_scenarios: Vec<FireScenario>,
    #[state(artifact)]
    pub fatigue_details: Vec<FatigueDetail>,
    #[state(artifact)]
    pub cold_formed: Vec<ColdFormedSheet>,
    #[state(artifact)]
    pub shells: Vec<AluminiumShell>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for En1999Artifact {
    fn default() -> Self {
        Self::from_snapshot(En1999Snapshot::default())
    }
}

impl From<En1999Snapshot> for En1999Artifact {
    fn from(snapshot: En1999Snapshot) -> Self {
        Self::from_snapshot(snapshot)
    }
}

impl En1999Artifact {
    pub fn to_snapshot(&self) -> En1999Snapshot {
        En1999Snapshot {
            annex: self.annex,
            materials: self.materials.clone(),
            sections: self.sections.clone(),
            members: self.members.clone(),
            connections: self.connections.clone(),
            fire_scenarios: self.fire_scenarios.clone(),
            fatigue_details: self.fatigue_details.clone(),
            cold_formed: self.cold_formed.clone(),
            shells: self.shells.clone(),
        }
    }

    pub fn from_snapshot(snapshot: En1999Snapshot) -> Self {
        Self {
            annex: snapshot.annex,
            materials: snapshot.materials,
            sections: snapshot.sections,
            members: snapshot.members,
            connections: snapshot.connections,
            fire_scenarios: snapshot.fire_scenarios,
            fatigue_details: snapshot.fatigue_details,
            cold_formed: snapshot.cold_formed,
            shells: snapshot.shells,
        }
    }

    pub fn set_snapshot(&mut self, snapshot: En1999Snapshot) {
        *self = Self::from_snapshot(snapshot);
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
pub fn en1999_artifact_schema_descriptor() -> framework_schema::ArtifactSchemaDescriptor {
    framework_schema::ArtifactSchemaDescriptor {
        id: "s.norm.en1999",
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
    use crate::{En1999Diff, En1999Mutation, En1999Snapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct En1999BuilderConstruction {
        snapshot: En1999Snapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for En1999BuilderConstruction {
        type Snapshot = En1999Snapshot;
        type Mutation = En1999Mutation;
        type Diff = En1999Diff;
        fn empty() -> Self {
            Self { snapshot: En1999Snapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<En1999Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<En1999Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <En1999Mutation as protocol::Mutation<En1999Snapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <En1999Diff as protocol::MutationDiff<En1999Snapshot>>::apply(&diff, &self.snapshot)?;
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
    use crate::En1999Snapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct En1999Parts {
        pub snapshot: Option<En1999Snapshot>,
    }

    pub struct En1999AnalyzerAnalysis;

    impl ArtifactAnalysis for En1999AnalyzerAnalysis {
        type Parts = En1999Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.norm.en1999", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = En1999Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <En1999Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <En1999Snapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec En1999BuilderFacets {
        construction: En1999BuilderConstruction,
        analysis: En1999AnalyzerAnalysis,
        composition: super::super::io::derived_composition::En1999ComposerComposition,
    }
    builder: En1999Builder,
    analyzer: En1999Analyzer,
    composer: En1999Composer,
);
//#endregion 🧬️DerivedArtifactFacets


//#region 🔖️ComplianceHelpers

fn q_force(n: f64) -> Quantity { Quantity::new(QuantityKind::Force, n) }
fn q_moment(nm: f64) -> Quantity { Quantity::new(QuantityKind::Moment, nm) }
fn q_stress(pa: f64) -> Quantity { Quantity::new(QuantityKind::Stress, pa) }
fn q_length(m: f64) -> Quantity { Quantity::new(QuantityKind::Length, m) }
fn q_temp(c: f64) -> Quantity { Quantity::new(QuantityKind::Temperature, c) }
fn q_time(s: f64) -> Quantity { Quantity::new(QuantityKind::Time, s) }
fn q_dim(v: f64) -> Quantity { Quantity::new(QuantityKind::Dimensionless, v) }
fn loc(en: &str, de: &str) -> LocalizedCopy { LocalizedCopy::new(en, de) }

pub mod na_de {
    use crate::document::AnnexChoice;
    pub const HAZ_ZONE_M: f64 = 0.025;
    pub const E_PA: f64 = 70_000.0e6;
    pub const NU: f64 = 0.33;

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct AnnexParams {
        pub choice: AnnexChoice,
        pub gamma_m1: f64,
        pub gamma_m2: f64,
    }

    impl AnnexParams {
        pub fn en() -> Self { Self { choice: AnnexChoice::En, gamma_m1: 1.1, gamma_m2: 1.25 } }
        /// 🇩️🇪️ DIN EN 1999-1-1/NA keeps γ_M1=1.10 and γ_M2=1.25 (no NDP override).
        pub fn de() -> Self { Self { choice: AnnexChoice::De, gamma_m1: 1.1, gamma_m2: 1.25 } }
        pub fn for_choice(choice: AnnexChoice) -> Self {
            match choice { AnnexChoice::En => Self::en(), AnnexChoice::De => Self::de() }
        }
    }
}


pub mod part_en1990 {
    use super::*;
    use crate::snapshot::{AluminiumMember, MemberAction};

    #[derive(Clone, Copy, Debug)]
    pub struct PsiRow { pub psi_0: f64, pub psi_1: f64, pub psi_2: f64 }

    pub fn psi_row(category: &str, annex: AnnexChoice) -> PsiRow {
        let de = matches!(annex, AnnexChoice::De);
        match category {
            "residential" | "A" | "office" | "B" | "self" => PsiRow { psi_0: 0.7, psi_1: 0.5, psi_2: 0.3 },
            "congregation" | "C" | "retail" | "D" => PsiRow { psi_0: 0.7, psi_1: 0.7, psi_2: 0.6 },
            "storage" | "E" => PsiRow { psi_0: 1.0, psi_1: 0.9, psi_2: 0.8 },
            "snow" => if de { PsiRow { psi_0: 0.5, psi_1: 0.2, psi_2: 0.0 } } else { PsiRow { psi_0: 0.5, psi_1: 0.2, psi_2: 0.0 } },
            "snow_high" => PsiRow { psi_0: 0.7, psi_1: 0.5, psi_2: 0.2 },
            "wind" => PsiRow { psi_0: 0.6, psi_1: 0.2, psi_2: 0.0 },
            "temperature" => PsiRow { psi_0: 0.6, psi_1: 0.5, psi_2: 0.0 },
            "fire" => PsiRow { psi_0: 0.0, psi_1: 0.0, psi_2: 0.0 },
            _ => PsiRow { psi_0: 0.7, psi_1: 0.5, psi_2: 0.3 },
        }
    }

    #[derive(Clone, Debug)]
    pub struct GoverningEffects {
        pub action_id: String,
        pub combination: &'static str,
        pub n_ed: f64,
        pub v_y_ed: f64,
        pub v_z_ed: f64,
        pub m_y_ed: f64,
        pub m_z_ed: f64,
    }

    fn span_factor(support: &str) -> (f64, f64) {
        // (moment coeff for wL², shear coeff for wL)
        match support {
            "cantilever" => (0.5, 1.0),
            "continuous" => (1.0 / 12.0, 0.5),
            _ => (1.0 / 8.0, 0.5), // simplySupported
        }
    }

    fn characteristic_effects(member: &AluminiumMember, a: &MemberAction) -> (f64, f64, f64, f64, f64) {
        if a.source == "udl" {
            let l = member.length.max(1e-6);
            let (km, kv) = span_factor(&member.support);
            let w = a.g_k_line + a.q_k_line;
            let m = w * l * l * km;
            let v = w * l * kv;
            (a.n_k, a.v_y_k, v + a.v_z_k, m + a.m_y_k, a.m_z_k)
        } else {
            (a.n_k, a.v_y_k, a.v_z_k, a.m_y_k, a.m_z_k)
        }
    }

    fn scale(e: (f64, f64, f64, f64, f64), f: f64) -> (f64, f64, f64, f64, f64) {
        (e.0 * f, e.1 * f, e.2 * f, e.3 * f, e.4 * f)
    }

    fn add(a: (f64, f64, f64, f64, f64), b: (f64, f64, f64, f64, f64)) -> (f64, f64, f64, f64, f64) {
        (a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3 + b.3, a.4 + b.4)
    }

    fn util_key(e: (f64, f64, f64, f64, f64)) -> f64 {
        e.0.abs() + e.1.abs() + e.2.abs() + e.3.abs() + e.4.abs()
    }

    /// ⚖️ EN 1990 (+ DE NA) ULS combination; picks governing case for member checks.
    pub fn governing_member_effects(member: &AluminiumMember, annex: AnnexChoice) -> GoverningEffects {
        let gamma_g = 1.35;
        let gamma_q = 1.50;
        let xi = 0.85;
        let permanents: Vec<_> = member.actions.iter().filter(|a| a.kind == "permanent").collect();
        let variables: Vec<_> = member.actions.iter().filter(|a| a.kind != "permanent" && a.kind != "fire").collect();

        let mut g = (0.0, 0.0, 0.0, 0.0, 0.0);
        for a in &permanents {
            g = add(g, characteristic_effects(member, a));
        }

        let mut best = GoverningEffects {
            action_id: permanents.first().map(|a| a.id.clone()).unwrap_or_else(|| "G".into()),
            combination: "uls-610b",
            n_ed: 0.0, v_y_ed: 0.0, v_z_ed: 0.0, m_y_ed: 0.0, m_z_ed: 0.0,
        };
        let mut best_key = -1.0;

        if variables.is_empty() {
            let e = scale(g, gamma_g);
            return GoverningEffects {
                action_id: best.action_id,
                combination: "uls-g",
                n_ed: e.0, v_y_ed: e.1, v_z_ed: e.2, m_y_ed: e.3, m_z_ed: e.4,
            };
        }

        for (i, lead) in variables.iter().enumerate() {
            let lead_e = characteristic_effects(member, lead);
            let psi_lead = psi_row(&lead.category, annex);
            // 6.10a: γ_G G + γ_Q ψ0 Q_lead + Σ γ_Q ψ0 Q_i
            let mut a610 = scale(g, gamma_g);
            a610 = add(a610, scale(lead_e, gamma_q * psi_lead.psi_0));
            for (j, other) in variables.iter().enumerate() {
                if i == j { continue; }
                let psi = psi_row(&other.category, annex);
                a610 = add(a610, scale(characteristic_effects(member, other), gamma_q * psi.psi_0));
            }
            // 6.10b: ξ γ_G G + γ_Q Q_lead + Σ γ_Q ψ0 Q_i
            let mut b610 = scale(g, xi * gamma_g);
            b610 = add(b610, scale(lead_e, gamma_q));
            for (j, other) in variables.iter().enumerate() {
                if i == j { continue; }
                let psi = psi_row(&other.category, annex);
                b610 = add(b610, scale(characteristic_effects(member, other), gamma_q * psi.psi_0));
            }
            for (combo, e) in [("uls-610a", a610), ("uls-610b", b610)] {
                let k = util_key(e);
                if k > best_key {
                    best_key = k;
                    best = GoverningEffects {
                        action_id: lead.id.clone(),
                        combination: combo,
                        n_ed: e.0, v_y_ed: e.1, v_z_ed: e.2, m_y_ed: e.3, m_z_ed: e.4,
                    };
                }
            }
        }
        best
    }

    /// 🔥 Accidental/fire combination: G + ψ2 Q (EN 1990 6.11b / fire).
    pub fn fire_member_effects(member: &AluminiumMember, annex: AnnexChoice) -> GoverningEffects {
        let permanents: Vec<_> = member.actions.iter().filter(|a| a.kind == "permanent").collect();
        let variables: Vec<_> = member.actions.iter().filter(|a| a.kind != "permanent" && a.kind != "fire").collect();
        let mut e = (0.0, 0.0, 0.0, 0.0, 0.0);
        for a in &permanents {
            e = add(e, characteristic_effects(member, a));
        }
        let mut lead_id = "G".to_string();
        for a in &variables {
            let psi = psi_row(&a.category, annex);
            e = add(e, scale(characteristic_effects(member, a), psi.psi_2));
            lead_id = a.id.clone();
        }
        GoverningEffects {
            action_id: lead_id,
            combination: "fire",
            n_ed: e.0, v_y_ed: e.1, v_z_ed: e.2, m_y_ed: e.3, m_z_ed: e.4,
        }
    }

    /// 🪟 EN 1990 SLS characteristic / frequent / quasi-permanent (DE NA ψ).
    pub fn sls_member_effects(member: &AluminiumMember, annex: AnnexChoice, kind: &str) -> GoverningEffects {
        let permanents: Vec<_> = member.actions.iter().filter(|a| a.kind == "permanent").collect();
        let variables: Vec<_> = member.actions.iter().filter(|a| a.kind != "permanent" && a.kind != "fire").collect();
        let mut g = (0.0, 0.0, 0.0, 0.0, 0.0);
        for a in &permanents {
            g = add(g, characteristic_effects(member, a));
        }
        if variables.is_empty() {
            return GoverningEffects {
                action_id: permanents.first().map(|a| a.id.clone()).unwrap_or_else(|| "G".into()),
                combination: match kind { "frequent" => "sls-freq", "quasiPermanent" => "sls-qp", _ => "sls-char" },
                n_ed: g.0, v_y_ed: g.1, v_z_ed: g.2, m_y_ed: g.3, m_z_ed: g.4,
            };
        }
        let mut best = GoverningEffects {
            action_id: variables[0].id.clone(),
            combination: "sls-char",
            n_ed: 0.0, v_y_ed: 0.0, v_z_ed: 0.0, m_y_ed: 0.0, m_z_ed: 0.0,
        };
        let mut best_key = -1.0;
        for (i, lead) in variables.iter().enumerate() {
            let lead_e = characteristic_effects(member, lead);
            let psi = psi_row(&lead.category, annex);
            let mut e = g;
            match kind {
                "frequent" => {
                    e = add(e, scale(lead_e, psi.psi_1));
                    for (j, other) in variables.iter().enumerate() {
                        if i == j { continue; }
                        let po = psi_row(&other.category, annex);
                        e = add(e, scale(characteristic_effects(member, other), po.psi_2));
                    }
                }
                "quasiPermanent" => {
                    e = add(e, scale(lead_e, psi.psi_2));
                    for (j, other) in variables.iter().enumerate() {
                        if i == j { continue; }
                        let po = psi_row(&other.category, annex);
                        e = add(e, scale(characteristic_effects(member, other), po.psi_2));
                    }
                }
                _ => {
                    // characteristic: G + Q_lead + Σ ψ0 Q_i
                    e = add(e, lead_e);
                    for (j, other) in variables.iter().enumerate() {
                        if i == j { continue; }
                        let po = psi_row(&other.category, annex);
                        e = add(e, scale(characteristic_effects(member, other), po.psi_0));
                    }
                }
            }
            let k = util_key(e);
            if k > best_key {
                best_key = k;
                best = GoverningEffects {
                    action_id: lead.id.clone(),
                    combination: match kind { "frequent" => "sls-freq", "quasiPermanent" => "sls-qp", _ => "sls-char" },
                    n_ed: e.0, v_y_ed: e.1, v_z_ed: e.2, m_y_ed: e.3, m_z_ed: e.4,
                };
            }
        }
        best
    }

    /// 🔗 Governing ULS effects for a connection from its own characteristic actions.
    pub fn governing_connection_effects(conn: &crate::snapshot::AluminiumConnection, annex: AnnexChoice) -> GoverningEffects {
        let proxy = AluminiumMember {
            id: conn.id.clone(),
            section_id: String::new(),
            material_id: conn.material_id.clone(),
            length: 1.0,
            support: "simplySupported".into(),
            buckling_length_y: 1.0,
            buckling_length_z: 1.0,
            buckling_length_t: 1.0,
            ltb_length: 1.0,
            c1: 1.0,
            restrained_ltb: true,
            actions: conn.actions.clone(),
        };
        governing_member_effects(&proxy, annex)
    }

}

pub mod part_1_1 {
    use super::*;
    use crate::snapshot::{AluminiumSection, PlateElement};

    /// 🔩 EN 1999-1-1 Table 3.2 alloy temper properties (extruded / sheet).
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct AlloyProps {
        pub f_o_pa: f64,
        pub f_u_pa: f64,
        pub buckling_class_a: bool,
        pub rho_o_haz: f64,
        pub rho_u_haz: f64,
    }

    pub const CATALOGUE_ALLOYS: &[&str] = &[
        "aw6060-t6", "aw6061-t6", "aw6063-t6", "aw6082-t6", "aw5083-o", "aw5083-h111",
    ];

    pub fn resolve_alloy(designation: &str) -> Option<AlloyProps> {
        Some(match designation.to_ascii_lowercase().replace('_', "-").as_str() {
            "aw6060-t6" | "aw6060t6" | "en-aw-6060-t6" => AlloyProps {
                f_o_pa: 160.0e6, f_u_pa: 215.0e6, buckling_class_a: true, rho_o_haz: 0.48, rho_u_haz: 0.56,
            },
            "aw6061-t6" | "aw6061t6" => AlloyProps {
                f_o_pa: 240.0e6, f_u_pa: 290.0e6, buckling_class_a: true, rho_o_haz: 0.53, rho_u_haz: 0.62,
            },
            "aw6063-t6" | "aw6063t6" => AlloyProps {
                f_o_pa: 170.0e6, f_u_pa: 215.0e6, buckling_class_a: true, rho_o_haz: 0.49, rho_u_haz: 0.56,
            },
            "aw6082-t6" | "aw6082t6" => AlloyProps {
                f_o_pa: 260.0e6, f_u_pa: 310.0e6, buckling_class_a: true, rho_o_haz: 0.64, rho_u_haz: 0.73,
            },
            "aw5083-o" | "aw5083-h111" | "aw5083o" | "aw5083h111" => AlloyProps {
                f_o_pa: 125.0e6, f_u_pa: 275.0e6, buckling_class_a: false, rho_o_haz: 1.0, rho_u_haz: 1.0,
            },
            _ => return None,
        })
    }

    pub fn epsilon(f_o_pa: f64) -> f64 { (250.0e6 / f_o_pa).sqrt() }

    pub fn eta_factor(welded: bool) -> f64 { if welded { 0.70 } else { 1.0 } }

    pub fn beta(element: &PlateElement) -> f64 {
        if element.thickness <= 0.0 { return f64::INFINITY; }
        eta_factor(element.welded) * element.width / element.thickness
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum SectionClass { Class1 = 1, Class2 = 2, Class3 = 3, Class4 = 4 }

    pub fn classify_element(element: &PlateElement, f_o_pa: f64) -> SectionClass {
        let eps = epsilon(f_o_pa);
        let b = beta(element) / eps;
        if element.outstand {
            if b <= 3.0 { SectionClass::Class1 }
            else if b <= 4.5 { SectionClass::Class2 }
            else if b <= 6.0 { SectionClass::Class3 }
            else { SectionClass::Class4 }
        } else if b <= 11.0 {
            SectionClass::Class1
        } else if b <= 16.0 {
            SectionClass::Class2
        } else if b <= 22.0 {
            SectionClass::Class3
        } else {
            SectionClass::Class4
        }
    }

    pub fn section_class(section: &AluminiumSection, f_o_pa: f64) -> SectionClass {
        section.elements.iter().map(|e| classify_element(e, f_o_pa)).max().unwrap_or(SectionClass::Class1)
    }

    /// 📉 Local buckling reduction ρ_c for class 4 elements (EN 1999-1-1 §6.1.5).
    pub fn local_buckling_rho(element: &PlateElement, f_o_pa: f64) -> f64 {
        let class = classify_element(element, f_o_pa);
        if class != SectionClass::Class4 { return 1.0; }
        let eps = epsilon(f_o_pa);
        let beta_bar = beta(element) / (eps * if element.outstand { 6.0 } else { 22.0 });
        if beta_bar <= 1.0 { 1.0 } else { (1.0 / beta_bar - 0.22 / (beta_bar * beta_bar)).clamp(0.0, 1.0) }
    }

    pub fn section_geometry(section: &AluminiumSection) -> (f64, f64, f64, f64, f64, f64, f64) {
        // returns A, Iy, Iz, Wel_y, Wel_z, It, Iw  (SI: m², m⁴, m³)
        match section.kind.as_str() {
            "tube" => {
                let d = section.outer_diameter.max(section.width);
                let t = section.web_thickness.max(section.flange_thickness);
                let di = (d - 2.0 * t).max(0.0);
                let a = std::f64::consts::PI / 4.0 * (d * d - di * di);
                let i = std::f64::consts::PI / 64.0 * (d.powi(4) - di.powi(4));
                let wel = if d > 0.0 { 2.0 * i / d } else { 0.0 };
                let it = 2.0 * std::f64::consts::PI * ((d - t) * 0.5).powi(3) * t;
                (a, i, i, wel, wel, it, 0.0)
            }
            "box" => {
                let h = section.height; let b = section.width; let tf = section.flange_thickness; let tw = section.web_thickness;
                let a = 2.0 * b * tf + 2.0 * (h - 2.0 * tf).max(0.0) * tw;
                let iy = (b * h.powi(3) - (b - 2.0 * tw).max(0.0) * (h - 2.0 * tf).max(0.0).powi(3)) / 12.0;
                let iz = (h * b.powi(3) - (h - 2.0 * tf).max(0.0) * (b - 2.0 * tw).max(0.0).powi(3)) / 12.0;
                let wel_y = if h > 0.0 { 2.0 * iy / h } else { 0.0 };
                let wel_z = if b > 0.0 { 2.0 * iz / b } else { 0.0 };
                let it = 2.0 * tf * tw * (h - tf).powi(2) * (b - tw).powi(2) / (h * tw + b * tf).max(1e-12);
                (a, iy, iz, wel_y, wel_z, it, 0.0)
            }
            "angle" => {
                let b = section.width; let h = section.height; let t = section.flange_thickness.max(section.web_thickness);
                let a = (b + h - t) * t;
                let iy = t * h.powi(3) / 12.0 + b * t.powi(3) / 12.0;
                let iz = t * b.powi(3) / 12.0 + h * t.powi(3) / 12.0;
                let wel_y = if h > 0.0 { iy / (h * 0.5) } else { 0.0 };
                let wel_z = if b > 0.0 { iz / (b * 0.5) } else { 0.0 };
                let it = (b + h - t) * t.powi(3) / 3.0;
                (a, iy, iz, wel_y, wel_z, it, 0.0)
            }
            _ => {
                // extruded I
                let h = section.height; let b = section.width; let tf = section.flange_thickness; let tw = section.web_thickness;
                let a = 2.0 * b * tf + (h - 2.0 * tf).max(0.0) * tw;
                let iy = (b * h.powi(3) - (b - tw).max(0.0) * (h - 2.0 * tf).max(0.0).powi(3)) / 12.0;
                let iz = 2.0 * (tf * b.powi(3) / 12.0) + (h - 2.0 * tf).max(0.0) * tw.powi(3) / 12.0;
                let wel_y = if h > 0.0 { 2.0 * iy / h } else { 0.0 };
                let wel_z = if b > 0.0 { 2.0 * iz / b } else { 0.0 };
                let it = (2.0 * b * tf.powi(3) + (h - 2.0 * tf).max(0.0) * tw.powi(3)) / 3.0;
                let iw = tf * b.powi(3) / 24.0 * ((h - tf) * 0.5).powi(2);
                (a, iy, iz, wel_y, wel_z, it, iw)
            }
        }
    }

    pub fn effective_area(section: &AluminiumSection, f_o_pa: f64, apply_haz: bool, rho_o_haz: f64, rho_u_haz: f64) -> f64 {
        let (a, _, _, _, _, _, _) = section_geometry(section);
        let mut a_eff = 0.0;
        for e in &section.elements {
            let mut rho = local_buckling_rho(e, f_o_pa);
            if apply_haz && e.welded {
                // §6.1.6 / §6.2.3–6.2.4: HAZ uses governing ρ_o,haz (proof) and ρ_u,haz (ultimate).
                let haz_factor = (1.0 + e.weld_position.max(0.0) / e.width.max(1e-6)).min(1.25);
                let rho_haz = rho_o_haz.min(rho_u_haz) / haz_factor.max(1.0);
                rho = rho.min(rho_haz.max(0.05));
            }
            a_eff += e.width * e.thickness * rho;
        }
        if a_eff <= 0.0 {
            a * (if apply_haz { rho_o_haz.min(rho_u_haz) } else { 1.0 })
        } else {
            a_eff
        }
    }

    pub fn effective_wel_y(section: &AluminiumSection, f_o_pa: f64, apply_haz: bool, rho_o_haz: f64, rho_u_haz: f64) -> f64 {
        let (_, _, _, wel_y, _, _, _) = section_geometry(section);
        let mut rho_min: f64 = 1.0;
        for e in &section.elements {
            rho_min = rho_min.min(local_buckling_rho(e, f_o_pa));
            if apply_haz && e.welded {
                let haz_factor = (1.0 + e.weld_position.max(0.0) / e.width.max(1e-6)).min(1.25);
                // §6.2.5 / HAZ bending uses ρ_o,haz (proof); ρ_u retained for tension callers.
                let _ = rho_u_haz;
                rho_min = rho_min.min(rho_o_haz / haz_factor.max(1.0));
            }
        }
        wel_y * rho_min
    }

    pub fn buckling_curve(class_a: bool) -> (f64, f64) {
        if class_a { (0.20, 0.10) } else { (0.32, 0.00) }
    }

    /// 📉 Flexural buckling reduction χ from λ̄ (EN 1999-1-1 §6.3.1) — never an input.
    pub fn chi_from_lambda(lambda_bar: f64, alpha: f64, lambda_0: f64) -> f64 {
        if lambda_bar <= lambda_0 { return 1.0; }
        let phi = 0.5 * (1.0 + alpha * (lambda_bar - lambda_0) + lambda_bar * lambda_bar);
        let denom = phi + (phi * phi - lambda_bar * lambda_bar).max(0.0).sqrt();
        if denom <= 0.0 { 1.0 } else { (1.0 / denom).min(1.0) }
    }

    pub fn lambda_bar(l_cr: f64, i: f64, a: f64, f_o_pa: f64) -> f64 {
        if i <= 0.0 || a <= 0.0 { return 0.0; }
        let i_rad = (i / a).sqrt();
        let lambda = l_cr / i_rad;
        let lambda_1 = std::f64::consts::PI * (na_de::E_PA / f_o_pa).sqrt();
        lambda / lambda_1
    }

    pub fn n_cr(e: f64, i: f64, l_cr: f64) -> f64 {
        if l_cr <= 0.0 { return f64::INFINITY; }
        std::f64::consts::PI.powi(2) * e * i / (l_cr * l_cr)
    }

    /// �フレーション Torsional buckling: N_cr,T ≈ G·I_t / i_0² + … simplified for open sections.
    pub fn torsional_lambda_bar(a: f64, i_t: f64, i_y: f64, i_z: f64, l_cr_t: f64, f_o_pa: f64) -> f64 {
        if a <= 0.0 || l_cr_t <= 0.0 { return 0.0; }
        let g = na_de::E_PA / (2.0 * (1.0 + na_de::NU));
        let i_0 = (i_y + i_z) / a;
        if i_0 <= 0.0 { return 0.0; }
        let n_cr_t = g * i_t / i_0;
        if n_cr_t <= 0.0 { return 10.0; }
        (a * f_o_pa / n_cr_t).sqrt()
    }

    /// 📐 Critical LTB moment M_cr for doubly-symmetric I (EN 1999-1-1 Annex I style).
    pub fn m_cr_ltb(c1: f64, e: f64, i_z: f64, i_w: f64, i_t: f64, l_lt: f64) -> f64 {
        if l_lt <= 0.0 || i_z <= 0.0 { return f64::INFINITY; }
        let g = e / (2.0 * (1.0 + na_de::NU));
        let term = (i_w / i_z + (l_lt * l_lt * g * i_t) / (std::f64::consts::PI.powi(2) * e * i_z)).max(0.0).sqrt();
        c1 * (std::f64::consts::PI.powi(2) * e * i_z / (l_lt * l_lt)) * term
    }

    pub fn shear_area(section: &AluminiumSection) -> f64 {
        match section.kind.as_str() {
            "tube" | "chs" => {
                let (a, _, _, _, _, _, _) = section_geometry(section);
                // EN 1999-1-1 §6.2.6 — CHS shear area ≈ 2 A / π
                2.0 * a / std::f64::consts::PI
            }
            _ => {
                let h = section.height; let tw = section.web_thickness; let tf = section.flange_thickness;
                (h - tf).max(0.0) * tw
            }
        }
    }

    pub fn bolt_fu_pa(material: &str) -> f64 {
        match material {
            "10.9" => 1000.0e6,
            "8.8" => 800.0e6,
            "5.6" => 500.0e6,
            "4.6" => 400.0e6,
            _ => 0.0, // unknown grade → zero resistance (forces Fail + OneOf)
        }
    }

    pub fn bolt_shear_resistance(bolts: &crate::snapshot::BoltGroup, gamma_m2: f64) -> f64 {
        let f_ub = bolt_fu_pa(&bolts.material);
        let a = std::f64::consts::PI / 4.0 * bolts.diameter * bolts.diameter;
        let n = (bolts.rows as f64) * (bolts.bolts_per_row as f64);
        let alpha_v = 0.6;
        n * alpha_v * f_ub * a / gamma_m2
    }

    pub fn bolt_bearing_resistance(bolts: &crate::snapshot::BoltGroup, f_u_pa: f64, gamma_m2: f64) -> f64 {
        let d = bolts.diameter;
        let t = bolts.plate_thickness;
        let e1 = bolts.edge_distance;
        let p1 = bolts.pitch;
        let alpha_d = (e1 / (3.0 * d)).min(p1 / (3.0 * d) - 0.25).min(1.0).max(0.0);
        let k1 = 2.5f64.min(2.8 * (bolts.gauge / d) - 1.7).min(2.5);
        let n = (bolts.rows as f64) * (bolts.bolts_per_row as f64);
        n * k1 * alpha_d * f_u_pa * d * t / gamma_m2
    }

    pub fn filler_fu_pa(filler: &str, parent_fu: f64) -> f64 {
        // EN 1999-1-1 Table 8.8 (simplified catalogue).
        match filler.to_ascii_lowercase().as_str() {
            "4043" | "alsi5" => 190.0e6,
            "5356" | "almg5" => 240.0e6,
            "5183" => 270.0e6,
            _ => parent_fu.min(190.0e6),
        }
    }

    pub fn haz_extent_m(welds: &crate::snapshot::WeldGroup, thickness: f64) -> f64 {
        // §6.1.6: b_haz grows with throat and plate thickness; stored hazExtent is additive.
        let base = (3.0 * welds.throat).max(thickness);
        base + welds.haz_extent.max(0.0)
    }

    pub fn weld_resistance(welds: &crate::snapshot::WeldGroup, parent_fu_pa: f64, gamma_m2: f64) -> f64 {
        let a_w = welds.throat * welds.length;
        let f_w = filler_fu_pa(&welds.filler_alloy, parent_fu_pa);
        a_w * f_w / (welds.beta_w.max(0.01) * gamma_m2)
    }
}

pub mod part_1_2 {
    use super::*;
    /// 🔥️ EN 1999-1-2 strength reduction k_θ (simplified table curve).
    pub fn k_theta(theta_a: f64) -> f64 {
        if theta_a <= 100.0 { 1.0 }
        else if theta_a >= 550.0 { 0.0 }
        else { (550.0 - theta_a) / 450.0 }
    }
    pub fn critical_temperature_c(f_o_pa: f64) -> f64 {
        170.0 + 0.4 * (f_o_pa / 1.0e6)
    }
}

pub mod part_1_3 {
    use super::*;
    /// N_C = 2e6 (Δσ_C reference), N_D = 5e6 (knee), N_L = 1e8 (cut-off).
    pub const N_C: f64 = 2.0e6;
    pub const N_D: f64 = 5.0e6;
    pub const N_L: f64 = 1.0e8;
    pub const GAMMA_MF_DE: f64 = 1.35;
    pub const GAMMA_MF_EN: f64 = 1.00;

    /// 📋 Annex J detail category → Δσ_C [Pa] (selected rows).
    pub fn detail_category_delta_sigma_c(category: &str) -> Option<f64> {
        match category.trim() {
            "14" => Some(14.0e6),
            "18" => Some(18.0e6),
            "20" => Some(20.0e6),
            "25" => Some(25.0e6),
            "28" => Some(28.0e6),
            "32" => Some(32.0e6),
            "36" => Some(36.0e6),
            "40" | "40-weld" => Some(40.0e6),
            "45" => Some(45.0e6),
            "50" => Some(50.0e6),
            "56" => Some(56.0e6),
            "63" => Some(63.0e6),
            "71" => Some(71.0e6),
            "80" => Some(80.0e6),
            "90" => Some(90.0e6),
            _ => None,
        }
    }

    pub fn gamma_mf(annex: AnnexChoice) -> f64 {
        match annex { AnnexChoice::De => GAMMA_MF_DE, AnnexChoice::En => GAMMA_MF_EN }
    }

    /// 📈 Bi-linear S–N: Δσ_R(N) with m1 below N_D and m2 up to N_L.
    pub fn fatigue_strength_pa(delta_sigma_c: f64, m1: f64, m2: f64, n_cycles: f64) -> f64 {
        if n_cycles <= 0.0 { return delta_sigma_c; }
        if n_cycles <= N_D {
            return delta_sigma_c * (N_C / n_cycles).powf(1.0 / m1.max(1e-6));
        }
        if n_cycles >= N_L {
            let at_nd = delta_sigma_c * (N_C / N_D).powf(1.0 / m1.max(1e-6));
            return at_nd * (N_D / N_L).powf(1.0 / m2.max(1e-6));
        }
        let at_nd = delta_sigma_c * (N_C / N_D).powf(1.0 / m1.max(1e-6));
        at_nd * (N_D / n_cycles).powf(1.0 / m2.max(1e-6))
    }

    /// 🧮 Damage sum D = n/N for one spectrum block (Miner).
    pub fn damage_ratio(delta_sigma: f64, delta_sigma_c: f64, m1: f64, m2: f64, n_cycles: f64, gamma_mf: f64) -> f64 {
        let rd = fatigue_strength_pa(delta_sigma_c, m1, m2, n_cycles) / gamma_mf.max(1e-6);
        if rd <= 0.0 { return 10.0; }
        let n_allow = if delta_sigma <= 0.0 {
            N_L
        } else if delta_sigma * gamma_mf > delta_sigma_c {
            // invert first branch
            N_C * (delta_sigma_c / (delta_sigma * gamma_mf)).powf(m1.max(1e-6))
        } else {
            let at_nd = delta_sigma_c * (N_C / N_D).powf(1.0 / m1.max(1e-6));
            if delta_sigma * gamma_mf > at_nd * (N_D / N_L).powf(1.0 / m2.max(1e-6)) {
                N_D * (at_nd / (delta_sigma * gamma_mf)).powf(m2.max(1e-6))
            } else {
                N_L
            }
        };
        n_cycles / n_allow.max(1e-9)
    }
}


fn elements_digest(section: &crate::snapshot::AluminiumSection) -> String {
    if section.elements.is_empty() {
        return format!("kind={k};D={d:.4};t={t:.4}", k=section.kind, d=section.outer_diameter, t=section.web_thickness);
    }
    section.elements.iter().map(|e| format!(
        "{id}:out={out}:weld={w}:wp={wp:.5}:b={b:.5}:t={th:.5}",
        id=e.id, out=e.outstand, w=e.welded, wp=e.weld_position, b=e.width, th=e.thickness
    )).collect::<Vec<_>>().join("|")
}

fn actions_digest(actions: &[crate::snapshot::MemberAction]) -> String {
    actions.iter().map(|a| format!(
        "{id}/{kind}/{cat}/{src}/g={g:.4}/q={q:.4}/N={n:.4}/Vy={vy:.4}/Vz={vz:.4}/My={my:.4}/Mz={mz:.4}",
        id=a.id, kind=a.kind, cat=a.category, src=a.source,
        g=a.g_k_line, q=a.q_k_line, n=a.n_k, vy=a.v_y_k, vz=a.v_z_k, my=a.m_y_k, mz=a.m_z_k
    )).collect::<Vec<_>>().join(" | ")
}

fn member_support_digest(member: &crate::snapshot::AluminiumMember) -> String {
    format!(
        "support={sup};c1={c1:.4};LcrY={ly:.4};LcrZ={lz:.4};LcrT={lt:.4};Llt={llt:.4};restrained={r}",
        sup=member.support, c1=member.c1, ly=member.buckling_length_y, lz=member.buckling_length_z,
        lt=member.buckling_length_t, llt=member.ltb_length, r=member.restrained_ltb
    )
}

fn member_label(id: &str) -> LocalizedCopy {
    loc(&format!("Member {id}"), &format!("Bauteil {id}"))
}
fn conn_label(id: &str) -> LocalizedCopy {
    loc(&format!("Connection {id}"), &format!("Anschluss {id}"))
}

fn utilization_check(
    id: String,
    part: &str,
    clause: ClauseId,
    subject: SubjectRef,
    title: LocalizedCopy,
    computed: Quantity,
    limit: Quantity,
    annex: AnnexChoice,
    explanation: LocalizedCopy,
    mut remedies: Vec<Remedy>,
) -> CheckResult {
    let u = if limit.value.abs() > 0.0 { computed.value.abs() / limit.value.abs() } else if computed.value.abs() > 0.0 { f64::INFINITY } else { 0.0 };
    if u > 1.0 && remedies.is_empty() && !subject.path.is_empty() {
        remedies.push(Remedy::at_most(
            subject.clone(),
            computed,
            Quantity::new(computed.kind, 0.0),
            loc("Reduce the governing demand until utilization ≤ 1.", "Maßgebende Beanspruchung senken, bis Ausnutzung ≤ 1."),
        ));
    }
    let mut b = CheckResult::assess(id, part, clause, subject, title)
        .utilization(computed, limit)
        .annex(annex)
        .explanation(explanation);
    for r in remedies { b = b.remedy(r); }
    b.build()
}

/// 📋️ Evaluate one member against EN 1999-1-1 cross-section and stability clauses.
pub fn check_member(
    member: &crate::snapshot::AluminiumMember,
    section: &AluminiumSection,
    material: &AluminiumMaterial,
    annex: AnnexChoice,
) -> Vec<CheckResult> {
    let params = na_de::AnnexParams::for_choice(annex);
    let Some(alloy) = part_1_1::resolve_alloy(&material.designation) else {
        return vec![CheckResult::assess(
            format!("en1999.3.2.alloy.{}", member.id),
            "DIN EN 1999-1-1",
            ClauseId::new("EN 1999-1-1", "§3.2", "3.2"),
            SubjectRef::new(&material.id, format!("materials[id={}].designation", material.id), loc("Alloy", "Legierung")),
            loc("Alloy designation", "Legierungsbezeichnung"),
        )
        .status(crate::document::CheckStatus::Fail)
        .explanation(loc(
            &format!("Unknown alloy designation '{}'.", material.designation),
            &format!("Unbekannte Legierungsbezeichnung '{}'.", material.designation),
        ))
        .annex(annex)
        .remedy(Remedy::one_of(
            SubjectRef::new(&material.id, format!("materials[id={}].designation", material.id), loc("Alloy", "Legierung")),
            part_1_1::CATALOGUE_ALLOYS.iter().map(|s| (*s).to_string()).collect(),
            loc("Pick a catalogue alloy from EN 1999-1-1 Table 3.2.", "Kataloglegierung nach EN 1999-1-1 Tabelle 3.2 wählen."),
        ))
        .build()];
    };
    let (a_gross, i_y, i_z, _wel_y, _wel_z, i_t, i_w) = part_1_1::section_geometry(section);
    let class = part_1_1::section_class(section, alloy.f_o_pa);
    let has_weld = section.elements.iter().any(|e| e.welded);
    let a_eff = part_1_1::effective_area(section, alloy.f_o_pa, has_weld, alloy.rho_o_haz, alloy.rho_u_haz);
    let wel_eff = part_1_1::effective_wel_y(section, alloy.f_o_pa, has_weld, alloy.rho_o_haz, alloy.rho_u_haz);
    let el_dig = elements_digest(section);
    let (alpha, lambda_0) = part_1_1::buckling_curve(alloy.buckling_class_a);
    let mut out = Vec::new();
    let part = "DIN EN 1999-1-1";

    // Classification informational (Pass if class <= 3, Warning/Fail messaging via utilization on beta)
    let governing = section.elements.iter().max_by(|a, b| {
        part_1_1::beta(a).partial_cmp(&part_1_1::beta(b)).unwrap_or(std::cmp::Ordering::Equal)
    });
    if let Some(el) = governing {
        let eps = part_1_1::epsilon(alloy.f_o_pa);
        let beta_eps = part_1_1::beta(el) / eps;
        let limit_beta = if el.outstand { 6.0 } else { 22.0 };
        let path = format!("sections[id={}].elements[id={}].thickness", section.id, el.id);
        let mut remedies = Vec::new();
        if beta_eps > limit_beta {
            let t_req = part_1_1::eta_factor(el.welded) * el.width / (limit_beta * eps);
            remedies.push(Remedy::at_least(
                SubjectRef::new(&section.id, path.clone(), loc(&format!("Plate {}", el.id), &format!("Blech {}", el.id))),
                q_length(el.thickness),
                q_length(t_req),
                loc(
                    &format!("Increase plate thickness of {} from {:.1} mm to at least {:.1} mm (class ≤ 3).", el.id, el.thickness * 1e3, t_req * 1e3),
                    &format!("Blechdicke von {} von {:.1} mm auf mindestens {:.1} mm erhöhen (Klasse ≤ 3).", el.id, el.thickness * 1e3, t_req * 1e3),
                ),
            ));
            remedies.push(Remedy::one_of(
                SubjectRef::new(&material.id, format!("materials[id={}].designation", material.id), loc("Alloy", "Legierung")),
                vec!["aw6082-t6".into(), "aw6061-t6".into(), "aw6063-t6".into()],
                loc("Select a higher-strength alloy/temper from Table 3.2.", "Höherfeste Legierung/Zustand nach Tabelle 3.2 wählen."),
            ));
        }
        out.push(utilization_check(
            format!("en1999.6.1.4.class.{}", member.id),
            part,
            ClauseId::new("EN 1999-1-1", "§6.1.4", "6.1.4"),
            SubjectRef::new(&member.id, path, member_label(&member.id)),
            loc("Cross-section classification (Table 6.2)", "Querschnittsklasse (Tabelle 6.2)"),
            q_dim(beta_eps),
            q_dim(limit_beta),
            annex,
            loc(
                &format!("Governing β/ε = {beta_eps:.2} vs limit {limit_beta:.1} (class {:?}); outstand={}; weldPos={:.5}; {el_dig}", class, el.outstand, el.weld_position),
                &format!("Maßgebendes β/ε = {beta_eps:.2} gegenüber Grenzwert {limit_beta:.1} (Klasse {:?}); auskragend={}; Schweißlage={:.5}; {el_dig}", class, el.outstand, el.weld_position),
            ),
            remedies,
        ));
    }

    let gov = part_en1990::governing_member_effects(member, annex);
    let action_id = gov.action_id.as_str();
    let (n_ed, v_y, v_z, m_y, m_z) = (gov.n_ed, gov.v_y_ed, gov.v_z_ed, gov.m_y_ed, gov.m_z_ed);
    let combo = gov.combination;
    let act_dig = actions_digest(&member.actions);
    let sup_dig = member_support_digest(member);
    let path_n = format!("members[id={}].actions[id={}].nK", member.id, action_id);
    let path_m = format!("members[id={}].actions[id={}].mYK", member.id, action_id);
    let path_mz = format!("members[id={}].actions[id={}].mZK", member.id, action_id);
    let path_vy = format!("members[id={}].actions[id={}].vYK", member.id, action_id);
    let l_cr_y = if member.buckling_length_y > 0.0 { member.buckling_length_y } else { member.length };
    let l_cr_z = if member.buckling_length_z > 0.0 { member.buckling_length_z } else { member.length };
    let l_cr_t = if member.buckling_length_t > 0.0 { member.buckling_length_t } else { member.length };
    let l_lt = if member.ltb_length > 0.0 { member.ltb_length } else { member.length };

    // Tension / compression §6.2.3 / 6.2.4
    let n_rd = a_eff * alloy.f_o_pa / params.gamma_m1;
    let mut remedies_n = Vec::new();
    if n_ed.abs() > n_rd {
        let a_req = n_ed.abs() * params.gamma_m1 / alloy.f_o_pa;
        remedies_n.push(Remedy::one_of(
            SubjectRef::new(&member.id, format!("members[id={}].sectionId", member.id), member_label(&member.id)),
            vec!["sec-i160".into(), "sec-i180".into(), "sec-box120".into()],
            loc(
                &format!("Select a larger section so A_eff ≥ {a_req:.2e} m² (N_Ed = {:.1} kN).", n_ed.abs() / 1e3),
                &format!("Größeren Querschnitt wählen, sodass A_eff ≥ {a_req:.2e} m² (N_Ed = {:.1} kN).", n_ed.abs() / 1e3),
            ),
        ));
        remedies_n.push(Remedy::one_of(
            SubjectRef::new(&material.id, format!("materials[id={}].designation", material.id), loc("Alloy", "Legierung")),
            vec!["aw6082-t6".into(), "aw6061-t6".into()],
            loc("Upgrade alloy/temper to raise f_o.", "Legierung/Zustand erhöhen, um f_o zu steigern."),
        ));
        remedies_n.push(Remedy::at_most(
            SubjectRef::new(&member.id, path_n.clone(), member_label(&member.id)),
            q_force(n_ed),
            q_force(n_rd.copysign(n_ed)),
            loc(
                &format!("Reduce |N_Ed| from {:.1} kN to at most {:.1} kN.", n_ed.abs()/1e3, n_rd/1e3),
                &format!("|N_Ed| von {:.1} kN auf höchstens {:.1} kN reduzieren.", n_ed.abs()/1e3, n_rd/1e3),
            ),
        ));
    }
    out.push(utilization_check(
        format!("en1999.6.2.3.n.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§6.2.3", "6.2.3"),
        SubjectRef::new(&member.id, path_n.clone(), member_label(&member.id)),
        loc("Axial resistance N_Rd (gross/net/HAZ)", "Normalkraftwiderstand N_Rd (brutto/netto/WEZ)"),
        q_force(n_ed.abs()),
        q_force(n_rd),
        annex,
        loc(
            &format!("N_Ed={:.1} kN, A_eff={:.1} mm², f_o={:.0} MPa, N_Rd={:.1} kN (ULS {combo}, lead {action_id}); {act_dig}; {sup_dig}; {el_dig}", n_ed/1e3, a_eff*1e6, alloy.f_o_pa/1e6, n_rd/1e3),
            &format!("N_Ed={:.1} kN, A_eff={:.1} mm², f_o={:.0} MPa, N_Rd={:.1} kN (GZT {combo}, führend {action_id}); {act_dig}; {sup_dig}; {el_dig}", n_ed/1e3, a_eff*1e6, alloy.f_o_pa/1e6, n_rd/1e3),
        ),
        remedies_n,
    ));

    // Bending §6.2.5
    let m_rd = wel_eff * alloy.f_o_pa / params.gamma_m1;
    let mut remedies_m = Vec::new();
    if m_y.abs() > m_rd {
        remedies_m.push(Remedy::one_of(
            SubjectRef::new(&member.id, format!("members[id={}].sectionId", member.id), member_label(&member.id)),
            vec!["sec-i160".into(), "sec-i200".into(), "sec-chs168".into()],
            loc("Select a section with larger W_el,y.", "Querschnitt mit größerem W_el,y wählen."),
        ));
        if m_rd > 0.0 {
            remedies_m.push(Remedy::at_most(
                SubjectRef::new(&member.id, path_m.clone(), member_label(&member.id)),
                q_moment(m_y),
                q_moment(m_rd.copysign(m_y)),
                loc(
                    &format!("Reduce |M_y,Ed| from {:.2} kNm to at most {:.2} kNm.", m_y.abs()/1e3, m_rd/1e3),
                    &format!("|M_y,Ed| von {:.2} kNm auf höchstens {:.2} kNm reduzieren.", m_y.abs()/1e3, m_rd/1e3),
                ),
            ));
        } else {
            remedies_m.push(Remedy::at_most(
                SubjectRef::new(&member.id, path_m.clone(), member_label(&member.id)),
                q_moment(m_y),
                q_moment(0.0),
                loc("Reduce |M_y,Ed| to zero until section resistance is available.", "|M_y,Ed| auf null senken, bis Querschnittstragfähigkeit vorliegt."),
            ));
        }
    }
    out.push(utilization_check(
        format!("en1999.6.2.5.m.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§6.2.5", "6.2.5"),
        SubjectRef::new(&member.id, path_m.clone(), member_label(&member.id)),
        loc("Bending resistance M_c,Rd", "Biegewiderstand M_c,Rd"),
        q_moment(m_y.abs()),
        q_moment(m_rd),
        annex,
        loc(
            &format!("M_y,Ed={:.2} kNm, W_eff={:.0} mm³, M_c,Rd={:.2} kNm (ULS {combo}, lead {action_id}); {act_dig}; {sup_dig}", m_y/1e3, wel_eff*1e9, m_rd/1e3),
            &format!("M_y,Ed={:.2} kNm, W_eff={:.0} mm³, M_c,Rd={:.2} kNm (GZT {combo}, führend {action_id}); {act_dig}; {sup_dig}", m_y/1e3, wel_eff*1e9, m_rd/1e3),
        ),
        remedies_m,
    ));

    // Shear §6.2.6
    let a_v = part_1_1::shear_area(section);
    let v_rd = a_v * alloy.f_o_pa / (params.gamma_m1 * 3.0f64.sqrt());
    // web buckling reduction when slender
    let web = section.elements.iter().find(|e| !e.outstand);
    let mut v_rd_b = v_rd;
    if let Some(w) = web {
        let hw_t = if w.thickness > 0.0 { w.width / w.thickness } else { 0.0 };
        if hw_t > 39.0 * part_1_1::epsilon(alloy.f_o_pa) {
            v_rd_b *= (39.0 * part_1_1::epsilon(alloy.f_o_pa) / hw_t).min(1.0);
        }
    }
    let mut remedies_v = Vec::new();
    if v_z.abs() > v_rd_b && v_rd_b > 0.0 {
        remedies_v.push(Remedy::one_of(
            SubjectRef::new(&member.id, format!("members[id={}].sectionId", member.id), member_label(&member.id)),
            vec!["sec-i160".into(), "sec-i180".into()],
            loc("Select a section with thicker web.", "Querschnitt mit dickeren Steg wählen."),
        ));
    }
    out.push(utilization_check(
        format!("en1999.6.2.6.v.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§6.2.6", "6.2.6"),
        SubjectRef::new(&member.id, format!("members[id={}].actions[id={}].vZK", member.id, action_id), member_label(&member.id)),
        loc("Shear resistance V_c,Rd (incl. web buckling)", "Querkraftwiderstand V_c,Rd (inkl. Stegbeulen)"),
        q_force(v_z.abs()),
        q_force(v_rd_b),
        annex,
        loc(
            &format!("V_z,Ed={:.1} kN, V_c,Rd={:.1} kN.", v_z/1e3, v_rd_b/1e3),
            &format!("V_z,Ed={:.1} kN, V_c,Rd={:.1} kN. (DE)", v_z/1e3, v_rd_b/1e3),
        ),
        remedies_v,
    ));


    // Bending about z §6.2.5 (weak axis)
    let (_, _, _, _, wel_z_geom, _, _) = part_1_1::section_geometry(section);
    let wel_z = if wel_z_geom > 0.0 {
        wel_z_geom
    } else if section.width > 0.0 {
        let (_, _, iz, _, _, _, _) = part_1_1::section_geometry(section);
        2.0 * iz / section.width
    } else {
        0.0
    };
    let m_rd_z = wel_z * alloy.f_o_pa / params.gamma_m1;
    let mut remedies_mz = Vec::new();
    if m_z.abs() > m_rd_z && m_rd_z > 0.0 {
        remedies_mz.push(Remedy::at_most(
            SubjectRef::new(&member.id, path_mz.clone(), member_label(&member.id)),
            q_moment(m_z),
            q_moment(m_rd_z.copysign(m_z)),
            loc(
                &format!("Reduce |M_z,Ed| to ≤ {:.2} kNm.", m_rd_z/1e3),
                &format!("|M_z,Ed| auf ≤ {:.2} kNm reduzieren.", m_rd_z/1e3),
            ),
        ));
    }
    out.push(utilization_check(
        format!("en1999.6.2.5.mz.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§6.2.5", "6.2.5"),
        SubjectRef::new(&member.id, path_mz.clone(), member_label(&member.id)),
        loc("Bending resistance M_z,c,Rd", "Biegewiderstand M_z,c,Rd"),
        q_moment(m_z.abs()),
        q_moment(m_rd_z.max(1e-9)),
        annex,
        loc(
            &format!("M_z,Ed={:.2} kNm from governing combination on member length L={:.2} m.", m_z/1e3, member.length),
            &format!("M_z,Ed={:.2} kNm aus maßgebender Kombination bei Bauteillänge L={:.2} m.", m_z/1e3, member.length),
        ),
        remedies_mz,
    ));

    // Shear V_y §6.2.6
    let mut remedies_vy = Vec::new();
    if v_y.abs() > v_rd_b && v_rd_b > 0.0 {
        remedies_vy.push(Remedy::at_most(
            SubjectRef::new(&member.id, path_vy.clone(), member_label(&member.id)),
            q_force(v_y),
            q_force(v_rd_b.copysign(v_y)),
            loc("Reduce |V_y,Ed|.", "|V_y,Ed| reduzieren."),
        ));
    }
    out.push(utilization_check(
        format!("en1999.6.2.6.vy.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§6.2.6", "6.2.6"),
        SubjectRef::new(&member.id, path_vy.clone(), member_label(&member.id)),
        loc("Shear resistance V_y,c,Rd", "Querkraftwiderstand V_y,c,Rd"),
        q_force(v_y.abs()),
        q_force(v_rd_b.max(1e-9)),
        annex,
        loc(
            &format!("V_y,Ed={:.1} kN (governing {action_id}).", v_y/1e3),
            &format!("V_y,Ed={:.1} kN (maßgebend {action_id}).", v_y/1e3),
        ),
        remedies_vy,
    ));

    // Combined §6.2.8 simplified
    let u_comb = if n_rd > 0.0 && m_rd > 0.0 { n_ed.abs()/n_rd + m_y.abs()/m_rd + (if m_rd_z > 0.0 { m_z.abs()/m_rd_z } else { 0.0 }) } else { 0.0 };
    let mut remedies_c = Vec::new();
    if u_comb > 1.0 {
        remedies_c.push(Remedy::one_of(
            SubjectRef::new(&member.id, format!("members[id={}].sectionId", member.id), member_label(&member.id)),
            vec!["sec-i160".into(), "sec-i200".into()],
            loc("Select a larger section to satisfy combined N–M.", "Größeren Querschnitt für N–M-Interaktion wählen."),
        ));
    }
    out.push(utilization_check(
        format!("en1999.6.2.8.nm.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§6.2.8", "6.2.8"),
        SubjectRef::new(&member.id, path_n.clone(), member_label(&member.id)),
        loc("Combined axial and bending", "Interaktion Normalkraft und Biegung"),
        q_dim(u_comb),
        q_dim(1.0),
        annex,
        loc(&format!("N_Ed/N_Rd + M_y,Ed/M_c,Rd = {u_comb:.3} (governing {action_id})."), &format!("N_Ed/N_Rd + M_y,Ed/M_c,Rd = {u_comb:.3} (maßgebend {action_id}).")),
        remedies_c,
    ));

    // Flexural buckling χ §6.3.1
    let lb_y = part_1_1::lambda_bar(l_cr_y, i_y, a_eff.max(a_gross), alloy.f_o_pa);
    let lb_z = part_1_1::lambda_bar(l_cr_z, i_z, a_eff.max(a_gross), alloy.f_o_pa);
    let chi_y = part_1_1::chi_from_lambda(lb_y, alpha, lambda_0);
    let chi_z = part_1_1::chi_from_lambda(lb_z, alpha, lambda_0);
    let chi = chi_y.min(chi_z);
    let n_b_rd = chi * a_eff * alloy.f_o_pa / params.gamma_m1;
    let mut remedies_b = Vec::new();
    if n_ed.abs() > n_b_rd && n_b_rd > 0.0 {
        // invert for L_cr: need chi' such that chi'*A*f/γ = |N|
        let chi_req = (n_ed.abs() * params.gamma_m1 / (a_eff * alloy.f_o_pa)).min(1.0);
        // binary search L_cr
        let mut lo = 0.1; let mut hi = member.buckling_length_y.max(member.buckling_length_z).max(0.5);
        for _ in 0..40 {
            let mid = 0.5 * (lo + hi);
            let lb = part_1_1::lambda_bar(mid, i_y.min(i_z), a_eff.max(a_gross), alloy.f_o_pa);
            let ch = part_1_1::chi_from_lambda(lb, alpha, lambda_0);
            if ch >= chi_req { lo = mid; } else { hi = mid; }
        }
        remedies_b.push(Remedy::at_most(
            SubjectRef::new(&member.id, format!("members[id={}].bucklingLengthY", member.id), member_label(&member.id)),
            q_length(member.buckling_length_y),
            q_length(lo),
            loc(
                &format!("Reduce buckling length L_cr,y from {:.2} m to at most {:.2} m (χ computed = {chi:.3}).", member.buckling_length_y, lo),
                &format!("Biegedrillknicklänge L_cr,y von {:.2} m auf höchstens {:.2} m verkürzen (χ = {chi:.3}).", member.buckling_length_y, lo),
            ),
        ));
        remedies_b.push(Remedy::one_of(
            SubjectRef::new(&member.id, format!("members[id={}].sectionId", member.id), member_label(&member.id)),
            vec!["sec-i160".into(), "sec-i200".into()],
            loc("Select a stockier section to raise χ·A.", "Stockigeren Querschnitt wählen, um χ·A zu erhöhen."),
        ));
    }
    out.push(utilization_check(
        format!("en1999.6.3.1.fb.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§6.3.1", "6.3.1"),
        SubjectRef::new(&member.id, format!("members[id={}].bucklingLengthY", member.id), member_label(&member.id)),
        loc("Flexural buckling N_b,Rd (χ computed)", "Biegeknicken N_b,Rd (χ berechnet)"),
        q_force(n_ed.abs()),
        q_force(n_b_rd),
        annex,
        loc(
            &format!("λ̄_y={lb_y:.3}, λ̄_z={lb_z:.3}, α={alpha}, λ̄_0={lambda_0}, χ={chi:.3}, N_b,Rd={:.1} kN.", n_b_rd/1e3),
            &format!("λ̄_y={lb_y:.3}, λ̄_z={lb_z:.3}, α={alpha}, λ̄_0={lambda_0}, χ={chi:.3}, N_b,Rd={:.1} kN. (DE)", n_b_rd/1e3),
        ),
        remedies_b,
    ));

    // Torsional / flexural-torsional
    let lb_t = part_1_1::torsional_lambda_bar(a_eff.max(a_gross), i_t, i_y, i_z, l_cr_t, alloy.f_o_pa);
    let chi_t = part_1_1::chi_from_lambda(lb_t, alpha, lambda_0);
    let n_t_rd = chi_t * a_eff * alloy.f_o_pa / params.gamma_m1;
    let mut remedies_t = Vec::new();
    if n_ed.abs() > n_t_rd && n_t_rd > 0.0 {
        remedies_t.push(Remedy::one_of(
            SubjectRef::new(&member.id, format!("members[id={}].sectionId", member.id), member_label(&member.id)),
            vec!["sec-box120".into(), "sec-tube114".into()],
            loc("Select a closed section to raise I_t.", "Geschlossenen Querschnitt wählen, um I_t zu erhöhen."),
        ));
        remedies_t.push(Remedy::at_most(
            SubjectRef::new(&member.id, format!("members[id={}].bucklingLengthT", member.id), member_label(&member.id)),
            q_length(l_cr_t),
            q_length(l_cr_t * 0.5),
            loc("Halve torsional buckling length L_cr,T.", "Torsionsknicklänge L_cr,T halbieren."),
        ));
    }
    out.push(utilization_check(
        format!("en1999.6.3.1.tb.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§6.3.1", "6.3.1.4"),
        SubjectRef::new(&member.id, format!("members[id={}].bucklingLengthT", member.id), member_label(&member.id)),
        loc("Torsional / flexural-torsional buckling", "Torsions- / Biegedrillknicken"),
        q_force(n_ed.abs()),
        q_force(n_t_rd),
        annex,
        loc(
            &format!("L_cr,T={l_cr_t:.4} m, λ̄_T={lb_t:.6}, χ_T={chi_t:.6}, N_t,Rd={:.1} kN; {sup_dig}", n_t_rd/1e3),
            &format!("L_cr,T={l_cr_t:.4} m, λ̄_T={lb_t:.6}, χ_T={chi_t:.6}, N_t,Rd={:.1} kN; {sup_dig} (Torsion)", n_t_rd/1e3),
        ),
        remedies_t,
    ));

    // LTB §6.3.2
    let m_cr = if member.restrained_ltb {
        f64::INFINITY
    } else {
        part_1_1::m_cr_ltb(member.c1, na_de::E_PA, i_z, i_w, i_t, l_lt)
    };
    let lambda_lt = if m_cr.is_finite() && m_cr > 0.0 { (wel_eff * alloy.f_o_pa / m_cr).sqrt() } else { 0.0 };
    let chi_lt = part_1_1::chi_from_lambda(lambda_lt, alpha, lambda_0);
    let m_b_rd = chi_lt * m_rd;
    let mut remedies_lt = Vec::new();
    if m_y.abs() > m_b_rd && m_b_rd > 0.0 {
        remedies_lt.push(Remedy::at_most(
            SubjectRef::new(&member.id, format!("members[id={}].ltbLength", member.id), member_label(&member.id)),
            q_length(l_lt),
            q_length(l_lt * 0.5),
            loc(
                &format!("Reduce LTB length from {:.2} m (M_cr={:.1} kNm, χ_LT={chi_lt:.3}).", l_lt, m_cr/1e3),
                &format!("Biegedrillknicklänge von {:.2} m verkürzen (M_cr={:.1} kNm, χ_LT={chi_lt:.3}).", l_lt, m_cr/1e3),
            ),
        ));
        remedies_lt.push(Remedy::one_of(
            SubjectRef::new(&member.id, format!("members[id={}].sectionId", member.id), member_label(&member.id)),
            vec!["sec-i160".into(), "sec-i200".into()],
            loc("Select a section with larger I_z / I_w.", "Querschnitt mit größerem I_z / I_w wählen."),
        ));
    }
    out.push(utilization_check(
        format!("en1999.6.3.2.ltb.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§6.3.2", "6.3.2"),
        SubjectRef::new(&member.id, format!("members[id={}].ltbLength", member.id), member_label(&member.id)),
        loc("Lateral-torsional buckling M_b,Rd", "Biegedrillknicken M_b,Rd"),
        q_moment(m_y.abs()),
        q_moment(m_b_rd),
        annex,
        loc(
            &format!("M_cr={:.1} kNm, λ̄_LT={lambda_lt:.6}, χ_LT={chi_lt:.6}, M_b,Rd={:.2} kNm; {sup_dig}; {act_dig}", m_cr/1e3, m_b_rd/1e3),
            &format!("M_cr={:.1} kNm, λ̄_LT={lambda_lt:.6}, χ_LT={chi_lt:.6}, M_b,Rd={:.2} kNm; {sup_dig}; {act_dig} (Biegedrillknicken)", m_cr/1e3, m_b_rd/1e3),
        ),
        remedies_lt,
    ));

    // Interaction 6.3.3
    let u_i = if n_b_rd > 0.0 && m_b_rd > 0.0 {
        n_ed.abs() / n_b_rd + m_y.abs() / m_b_rd
    } else { 0.0 };
    let mut remedies_i = Vec::new();
    if u_i > 1.0 {
        remedies_i.push(Remedy::one_of(
            SubjectRef::new(&member.id, format!("members[id={}].sectionId", member.id), member_label(&member.id)),
            vec!["sec-i160".into(), "sec-i200".into()],
            loc("Select a larger section for member buckling interaction.", "Größeren Querschnitt für Knickinteraktion wählen."),
        ));
        remedies_i.push(Remedy::at_most(
            SubjectRef::new(&member.id, format!("members[id={}].bucklingLengthY", member.id), member_label(&member.id)),
            q_length(member.buckling_length_y),
            q_length(member.buckling_length_y * 0.6),
            loc("Shorten buckling / LTB lengths (add restraints).", "Knick-/Biegedrillknicklängen verkürzen (Halterungen)."),
        ));
    }
    out.push(utilization_check(
        format!("en1999.6.3.3.int.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§6.3.3", "6.3.3"),
        SubjectRef::new(&member.id, path_n, member_label(&member.id)),
        loc("Member buckling interaction", "Interaktion Stabknicken"),
        q_dim(u_i),
        q_dim(1.0),
        annex,
        loc(
            &format!("N_Ed/N_b,Rd + M_Ed/M_b,Rd = {u_i:.3} (ULS {combo}, lead {action_id})."),
            &format!("N_Ed/N_b,Rd + M_Ed/M_b,Rd = {u_i:.3} (GZT {combo}, führend {action_id})."),
        ),
        remedies_i,
    ));

    // SLS deflection §7.2 (quasi-permanent) + elastic stress (characteristic)
    let sls_qp = part_en1990::sls_member_effects(member, annex, "quasiPermanent");
    let sls_char = part_en1990::sls_member_effects(member, annex, "characteristic");
    let (_, iy, _, _, _, _, _) = part_1_1::section_geometry(section);
    let e_mod = na_de::E_PA;
    let w_qp = if e_mod * iy > 0.0 && member.length > 0.0 {
        // simply-supported UDL proxy from |M| ≈ q L²/8 → q = 8M/L²; w = 5 q L⁴ / (384 EI)
        let m_qp = sls_qp.m_y_ed.abs();
        let q = 8.0 * m_qp / member.length.powi(2);
        5.0 * q * member.length.powi(4) / (384.0 * e_mod * iy)
    } else { 0.0 };
    let w_lim = member.length / 200.0; // EN 1999-1-1 §7.2 / DE NA vertical deflection
    let mut rem_w = Vec::new();
    if w_qp > w_lim {
        if member.length <= 0.0 {
            rem_w.push(Remedy::at_least(
                SubjectRef::new(&member.id, format!("members[id={}].length", member.id), member_label(&member.id)),
                q_length(member.length),
                q_length(1.0),
                loc("Set a positive member length for SLS deflection.", "Positive Bauteillänge für GZG-Durchbiegung setzen."),
            ));
        } else if w_lim > 0.0 {
            rem_w.push(Remedy::at_most(
                SubjectRef::new(&member.id, format!("members[id={}].actions[id={}].mYK", member.id, sls_qp.action_id), member_label(&member.id)),
                q_moment(sls_qp.m_y_ed.abs()),
                q_moment(sls_qp.m_y_ed.abs() * w_lim / w_qp.max(1e-12)),
                loc("Reduce quasi-permanent moment so deflection ≤ L/200.", "Quasi-ständiges Moment senken, damit Durchbiegung ≤ L/200."),
            ));
        }
    }
    out.push(utilization_check(
        format!("en1999.7.2.defl.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§7.2", "7.2"),
        SubjectRef::new(&member.id, format!("members[id={}].length", member.id), member_label(&member.id)),
        loc("SLS deflection (quasi-permanent)", "GZG Durchbiegung (quasi-ständig)"),
        q_length(w_qp),
        q_length(w_lim.max(1e-9)),
        annex,
        loc(
            &format!("w={:.1} mm ≤ L/200={:.1} mm (SLS {comb}, lead {aid}); {act_dig}; {sup_dig}", w_qp*1e3, w_lim*1e3, comb=sls_qp.combination, aid=sls_qp.action_id),
            &format!("w={:.1} mm ≤ L/200={:.1} mm (GZG {comb}, führend {aid}); {act_dig}; {sup_dig}", w_qp*1e3, w_lim*1e3, comb=sls_qp.combination, aid=sls_qp.action_id),
        ),
        rem_w,
    ));
    let sigma_sls = if wel_eff > 0.0 { sls_char.m_y_ed.abs() / wel_eff } else { 0.0 };
    let sigma_lim = 0.8 * alloy.f_o_pa; // elastic SLS stress limit
    let mut rem_s = Vec::new();
    if sigma_sls > sigma_lim {
        rem_s.push(Remedy::at_most(
            SubjectRef::new(&member.id, format!("members[id={}].actions[id={}].mYK", member.id, sls_char.action_id), member_label(&member.id)),
            q_moment(sls_char.m_y_ed.abs()),
            q_moment(if wel_eff > 0.0 { sigma_lim * wel_eff } else { 0.0 }),
            loc("Reduce characteristic SLS moment so σ ≤ 0.8 f_o.", "Charakteristisches GZG-Moment senken, damit σ ≤ 0,8 f_o."),
        ));
    }
    out.push(utilization_check(
        format!("en1999.7.2.stress.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§7.2", "7.2"),
        SubjectRef::new(&member.id, format!("members[id={}].actions[id={}].mYK", member.id, sls_char.action_id), member_label(&member.id)),
        loc("SLS elastic stress (characteristic)", "GZG elastische Spannung (charakteristisch)"),
        q_stress(sigma_sls),
        q_stress(sigma_lim.max(1e-9)),
        annex,
        loc(
            &format!("σ={:.0} MPa ≤ 0.8 f_o={:.0} MPa (SLS {comb}, lead {aid}); {act_dig}; {sup_dig}; {el_dig}", sigma_sls/1e6, sigma_lim/1e6, comb=sls_char.combination, aid=sls_char.action_id),
            &format!("σ={:.0} MPa ≤ 0,8 f_o={:.0} MPa (GZG {comb}, führend {aid}); {act_dig}; {sup_dig}; {el_dig}", sigma_sls/1e6, sigma_lim/1e6, comb=sls_char.combination, aid=sls_char.action_id),
        ),
        rem_s,
    ));

    out
}

pub fn check_connection(conn: &AluminiumConnection, material: &AluminiumMaterial, member: Option<&crate::snapshot::AluminiumMember>, annex: AnnexChoice) -> Vec<CheckResult> {
    let params = na_de::AnnexParams::for_choice(annex);
    let Some(alloy) = part_1_1::resolve_alloy(&material.designation) else {
        return vec![CheckResult::assess(
            format!("en1999.3.2.alloy.{}", conn.id),
            "DIN EN 1999-1-1",
            ClauseId::new("EN 1999-1-1", "§3.2", "3.2"),
            SubjectRef::new(&material.id, format!("materials[id={}].designation", material.id), loc("Alloy", "Legierung")),
            loc("Alloy designation", "Legierungsbezeichnung"),
        )
        .status(crate::document::CheckStatus::Fail)
        .explanation(loc(
            &format!("Unknown alloy designation '{}'.", material.designation),
            &format!("Unbekannte Legierungsbezeichnung '{}'.", material.designation),
        ))
        .annex(annex)
        .remedy(Remedy::one_of(
            SubjectRef::new(&material.id, format!("materials[id={}].designation", material.id), loc("Alloy", "Legierung")),
            part_1_1::CATALOGUE_ALLOYS.iter().map(|s| (*s).to_string()).collect(),
            loc("Pick a catalogue alloy from EN 1999-1-1 Table 3.2.", "Kataloglegierung nach EN 1999-1-1 Tabelle 3.2 wählen."),
        ))
        .build()];
    };
    let part = "DIN EN 1999-1-1";
    let mut out = Vec::new();
    let kind = conn.kind.to_ascii_lowercase();

    let gov = if conn.actions.is_empty() {
        member.map(|m| part_en1990::governing_member_effects(m, annex)).unwrap_or(part_en1990::GoverningEffects {
            action_id: "G".into(), combination: "uls-g", n_ed: 0.0, v_y_ed: 0.0, v_z_ed: 0.0, m_y_ed: 0.0, m_z_ed: 0.0,
        })
    } else {
        part_en1990::governing_connection_effects(conn, annex)
    };
    let combo = gov.combination;
    let action_id = gov.action_id.as_str();
    let n_ed = gov.n_ed;
    let v_ed = gov.v_z_ed.abs().max(gov.v_y_ed.abs());
    let m_ed = gov.m_y_ed.abs().max(gov.m_z_ed.abs());
    let conn_dig = actions_digest(&conn.actions);
    // memberId must resolve for connection checks that use governing effects
    if member.is_none() {
        return vec![CheckResult::assess(
            format!("en1999.8.ref.{}", conn.id),
            "DIN EN 1999-1-1",
            ClauseId::new("EN 1999-1-1", "§8", "8"),
            SubjectRef::new(&conn.id, format!("connections[id={}].memberId", conn.id), loc("Connection", "Anschluss")),
            loc("Connection member reference", "Anschluss-Bauteilverweis"),
        )
        .status(crate::document::CheckStatus::Fail)
        .explanation(loc(
            &format!("Connection '{}' references missing member '{}'.", conn.id, conn.member_id),
            &format!("Anschluss '{}' verweist auf fehlendes Bauteil '{}'.", conn.id, conn.member_id),
        ))
        .annex(annex)
        .remedy(Remedy::one_of(
            SubjectRef::new(&conn.id, format!("connections[id={}].memberId", conn.id), conn_label(&conn.id)),
            vec![conn.member_id.clone()],
            loc("Point connection.memberId at an existing member.", "connections.memberId auf ein vorhandenes Bauteil setzen."),
        ))
        .build()];
    }

    if kind == "bolted" || kind == "combined" {
        if part_1_1::bolt_fu_pa(&conn.bolts.material) <= 0.0 {
            out.push(CheckResult::assess(
                format!("en1999.8.5.boltgrade.{}", conn.id),
                part,
                ClauseId::new("EN 1999-1-1", "§8.5", "8.5"),
                SubjectRef::new(&conn.id, format!("connections[id={}].bolts.material", conn.id), conn_label(&conn.id)),
                loc("Bolt grade", "Schraubenfestigkeit"),
            )
            .status(crate::document::CheckStatus::Fail)
            .explanation(loc(
                &format!("Unknown bolt grade '{}'.", conn.bolts.material),
                &format!("Unbekannte Schraubenfestigkeit '{}'.", conn.bolts.material),
            ))
            .annex(annex)
            .remedy(Remedy::one_of(
                SubjectRef::new(&conn.id, format!("connections[id={}].bolts.material", conn.id), conn_label(&conn.id)),
                vec!["4.6".into(), "5.6".into(), "8.8".into(), "10.9".into()],
                loc("Select a bolt grade from EN 1999-1-1 Table 3.4.", "Schraubenfestigkeit nach EN 1999-1-1 Tabelle 3.4 wählen."),
            ))
            .build());
        }
        let f_v = part_1_1::bolt_shear_resistance(&conn.bolts, params.gamma_m2);
        let f_b = part_1_1::bolt_bearing_resistance(&conn.bolts, alloy.f_u_pa, params.gamma_m2);
        let f_rd = f_v.min(f_b);
        let mut remedies = Vec::new();
        if v_ed > f_rd && f_rd > 0.0 {
            let n_req = ((conn.bolts.rows * conn.bolts.bolts_per_row) as f64 * v_ed / f_rd).ceil() as u32;
            remedies.push(Remedy::at_least(
                SubjectRef::new(&conn.id, format!("connections[id={}].bolts.boltsPerRow", conn.id), conn_label(&conn.id)),
                q_dim(conn.bolts.bolts_per_row as f64),
                q_dim((n_req / conn.bolts.rows.max(1)).max(conn.bolts.bolts_per_row as u32 + 1) as f64),
                loc(
                    &format!("Increase bolt count (rows×bolts) so F_Rd ≥ {:.1} kN.", v_ed/1e3),
                    &format!("Schraubenzahl erhöhen, sodass F_Rd ≥ {:.1} kN.", v_ed/1e3),
                ),
            ));
        }
        if conn.bolts.edge_distance < 1.2 * conn.bolts.diameter {
            remedies.push(Remedy::at_least(
                SubjectRef::new(&conn.id, format!("connections[id={}].bolts.edgeDistance", conn.id), conn_label(&conn.id)),
                q_length(conn.bolts.edge_distance),
                q_length(1.2 * conn.bolts.diameter),
                loc("Increase edge distance e1 to ≥ 1.2 d.", "Randabstand e1 auf ≥ 1.2 d erhöhen."),
            ));
        }
        out.push(utilization_check(
            format!("en1999.8.5.bolt.{}", conn.id),
            part,
            ClauseId::new("EN 1999-1-1", "§8.5", "8.5"),
            SubjectRef::new(&conn.id, format!("connections[id={}].vK", conn.id), conn_label(&conn.id)),
            loc("Bolted connection resistance", "Schraubenanschluss Tragfähigkeit"),
            q_force(v_ed.max(n_ed.abs())),
            q_force(f_rd),
            annex,
            loc(
                &format!("V_Ed={:.1} kN, N_Ed={:.1} kN, M_Ed={:.2} kNm, F_v,Rd={:.1} kN, F_b,Rd={:.1} kN, d={d:.4} m, e1={e1:.4} m, p1={p1:.4} m, e2={e2:.4} m (ULS {combo}, lead {action_id}, bolt {mat}); {conn_dig}", v_ed/1e3, n_ed.abs()/1e3, m_ed/1e3, f_v/1e3, f_b/1e3, d=conn.bolts.diameter, e1=conn.bolts.edge_distance, p1=conn.bolts.pitch, e2=conn.bolts.gauge, mat=conn.bolts.material),
                &format!("V_Ed={:.1} kN, N_Ed={:.1} kN, M_Ed={:.2} kNm, F_v,Rd={:.1} kN, F_b,Rd={:.1} kN, d={d:.4} m, e1={e1:.4} m, p1={p1:.4} m, e2={e2:.4} m (GZT {combo}, führend {action_id}, Schraube {mat}); {conn_dig}", v_ed/1e3, n_ed.abs()/1e3, m_ed/1e3, f_v/1e3, f_b/1e3, d=conn.bolts.diameter, e1=conn.bolts.edge_distance, p1=conn.bolts.pitch, e2=conn.bolts.gauge, mat=conn.bolts.material),
            ),
            remedies,
        ));
    }

    if kind == "welded" || kind == "combined" {
        let f_w = part_1_1::weld_resistance(&conn.welds, alloy.f_u_pa, params.gamma_m2) * (alloy.rho_u_haz.min(alloy.rho_o_haz) / (1.0 + part_1_1::haz_extent_m(&conn.welds, conn.bolts.plate_thickness).max(conn.welds.throat) * 10.0)).clamp(0.2, 1.0);
        let mut remedies = Vec::new();
        if v_ed > f_w && f_w > 0.0 {
            let a_req = v_ed * conn.welds.beta_w * params.gamma_m2 / alloy.f_u_pa;
            let t_req = a_req / conn.welds.length.max(1e-6);
            remedies.push(Remedy::at_least(
                SubjectRef::new(&conn.id, format!("connections[id={}].welds.throat", conn.id), conn_label(&conn.id)),
                q_length(conn.welds.throat),
                q_length(t_req),
                loc(
                    &format!("Increase weld throat from {:.1} mm to at least {:.1} mm.", conn.welds.throat*1e3, t_req*1e3),
                    &format!("Kehlnahtdicke von {:.1} mm auf mindestens {:.1} mm erhöhen.", conn.welds.throat*1e3, t_req*1e3),
                ),
            ));
            let l_req = a_req / conn.welds.throat.max(1e-6);
            remedies.push(Remedy::at_least(
                SubjectRef::new(&conn.id, format!("connections[id={}].welds.length", conn.id), conn_label(&conn.id)),
                q_length(conn.welds.length),
                q_length(l_req),
                loc(
                    &format!("Increase weld length from {:.0} mm to at least {:.0} mm (or relocate weld).", conn.welds.length*1e3, l_req*1e3),
                    &format!("Nahtlänge von {:.0} mm auf mindestens {:.0} mm erhöhen (oder Naht verlegen).", conn.welds.length*1e3, l_req*1e3),
                ),
            ));
        }
        out.push(utilization_check(
            format!("en1999.8.6.weld.{}", conn.id),
            part,
            ClauseId::new("EN 1999-1-1", "§8.6", "8.6"),
            SubjectRef::new(&conn.id, format!("connections[id={}].welds.throat", conn.id), conn_label(&conn.id)),
            loc("Welded connection resistance", "Schweißanschluss Tragfähigkeit"),
            q_force(v_ed.max(n_ed.abs())),
            q_force(f_w),
            annex,
            loc(
                &format!("V_Ed={:.1} kN, N_Ed={:.1} kN, M_Ed={:.2} kNm, F_w,Rd={:.1} kN, a={:.1} mm, ℓ={:.0} mm, HAZ={:.0} mm, βw={bw:.3}, filler={filler} (ULS {combo}, lead {action_id}); {conn_dig}", v_ed/1e3, n_ed.abs()/1e3, m_ed/1e3, f_w/1e3, conn.welds.throat*1e3, conn.welds.length*1e3, conn.welds.haz_extent*1e3, bw=conn.welds.beta_w, filler=conn.welds.filler_alloy),
                &format!("V_Ed={:.1} kN, N_Ed={:.1} kN, M_Ed={:.2} kNm, F_w,Rd={:.1} kN, a={:.1} mm, ℓ={:.0} mm, WEZ={:.0} mm, βw={bw:.3}, Schweißzusatz={filler} (GZT {combo}, führend {action_id}); {conn_dig}", v_ed/1e3, n_ed.abs()/1e3, m_ed/1e3, f_w/1e3, conn.welds.throat*1e3, conn.welds.length*1e3, conn.welds.haz_extent*1e3, bw=conn.welds.beta_w, filler=conn.welds.filler_alloy),
            ),
            remedies,
        ));
    }

    if out.is_empty() {
        out.push(
            CheckResult::assess(
                format!("en1999.8.na.{}", conn.id),
                part,
                ClauseId::new("EN 1999-1-1", "§8", "8"),
                SubjectRef::new(&conn.id, format!("connections[id={}].kind", conn.id), conn_label(&conn.id)),
                loc("Connection check", "Anschlussnachweis"),
            )
            .not_applicable(loc("Connection kind not bolted/welded.", "Anschlussart weder geschraubt noch geschweißt."))
            .annex(annex)
            .build(),
        );
    }
    out
}

pub fn check_fire(
    fire: &FireScenario,
    member: &crate::snapshot::AluminiumMember,
    section: &AluminiumSection,
    material: &AluminiumMaterial,
    annex: AnnexChoice,
) -> CheckResult {
    let Some(alloy) = part_1_1::resolve_alloy(&material.designation) else {
        return CheckResult::assess(
            format!("en1999.1-2.fire.{}", fire.id),
            "DIN EN 1999-1-2",
            ClauseId::new("EN 1999-1-2", "§4", "4.2"),
            SubjectRef::new(&material.id, format!("materials[id={}].designation", material.id), loc("Alloy", "Legierung")),
            loc("Fire resistance", "Brandschutz"),
        )
        .status(crate::document::CheckStatus::Fail)
        .explanation(loc(
            &format!("Unknown alloy designation '{}'.", material.designation),
            &format!("Unbekannte Legierungsbezeichnung '{}'.", material.designation),
        ))
        .annex(annex)
        .remedy(Remedy::one_of(
            SubjectRef::new(&material.id, format!("materials[id={}].designation", material.id), loc("Alloy", "Legierung")),
            part_1_1::CATALOGUE_ALLOYS.iter().map(|s| (*s).to_string()).collect(),
            loc("Pick a catalogue alloy from EN 1999-1-1 Table 3.2.", "Kataloglegierung nach EN 1999-1-1 Tabelle 3.2 wählen."),
        ))
        .build();
    };
    let params = na_de::AnnexParams::for_choice(annex);
    let gov = part_en1990::fire_member_effects(member, annex);
    let n_ed = gov.n_ed.abs();
    let m_ed = gov.m_y_ed.abs();
    // EN 1999-1-2 §4.2: θ_a plus ISO-curve heating increment from duration.
    let theta_from_duration = 20.0 + 345.0 * ((1.0 + fire.duration_s.max(0.0) / 60.0).log10());
    let theta_eff = fire.theta_a + (theta_from_duration - 20.0).max(0.0) * 0.25;
    let k = part_1_2::k_theta(theta_eff);
    let theta_cr = part_1_2::critical_temperature_c(alloy.f_o_pa);
    let (a_gross, _iy, _iz, wel_y, _wel_z, _it, _iw) = part_1_1::section_geometry(section);
    let n_rd = a_gross * alloy.f_o_pa / params.gamma_m1;
    let m_rd = wel_y * alloy.f_o_pa / params.gamma_m1;
    let n_fi = k * n_rd;
    let m_fi = k * m_rd;
    let u_n = if n_fi > 0.0 { n_ed / n_fi } else if n_ed > 0.0 { f64::INFINITY } else { 0.0 };
    let u_m = if m_fi > 0.0 { m_ed / m_fi } else if m_ed > 0.0 { f64::INFINITY } else { 0.0 };
    let u = u_n.max(u_m);
    let mut remedies = Vec::new();
    if u > 1.0 {
        let k_req = if u_n >= u_m {
            if n_rd > 0.0 { (n_ed / n_rd).min(1.0) } else { 0.0 }
        } else if m_rd > 0.0 {
            (m_ed / m_rd).min(1.0)
        } else {
            0.0
        };
        let theta_req = if k_req >= 1.0 { 100.0 } else { (550.0 - 450.0 * k_req).clamp(100.0, 550.0) };
        remedies.push(Remedy::at_most(
            SubjectRef::new(&fire.id, format!("fireScenarios[id={}].thetaA", fire.id), loc(&format!("Fire {}", fire.id), &format!("Brand {}", fire.id))),
            q_temp(fire.theta_a),
            q_temp(theta_req),
            loc(
                &format!("Lower θ_a to ≤ {:.0} °C so k_θ≥{k_req:.3} restores fire resistance.", theta_req),
                &format!("θ_a auf ≤ {:.0} °C senken, damit k_θ≥{k_req:.3} den Brandwiderstand wiederherstellt.", theta_req),
            ),
        ));
        remedies.push(Remedy::at_most(
            SubjectRef::new(&fire.id, format!("fireScenarios[id={}].durationS", fire.id), loc("Fire duration", "Branddauer")),
            Quantity::new(QuantityKind::Time, fire.duration_s),
            Quantity::new(QuantityKind::Time, 600.0),
            loc("Shorten fire exposure duration.", "Branddauer verkürzen."),
        ));
    }
    utilization_check(
        format!("en1999.1-2.fire.{}", fire.id),
        "DIN EN 1999-1-2",
        ClauseId::new("EN 1999-1-2", "§4", "4.2"),
        SubjectRef::new(&fire.id, format!("fireScenarios[id={}].thetaA", fire.id), loc(&format!("Fire {}", fire.id), &format!("Brand {}", fire.id))),
        loc("Fire resistance with k_θ(θ_a)", "Brandschutz mit k_θ(θ_a)"),
        q_dim(u),
        q_dim(1.0),
        annex,
        loc(
            &format!(
                "θ_eff={:.0} °C (duration {:.0} s), θ_cr={:.0} °C, k_θ={k:.3}, N_Ed={:.1} kN / N_fi,Rd={:.1} kN, M_y,Ed={:.2} kNm / M_fi,Rd={:.2} kNm, u={u:.3}.",
                theta_eff, fire.duration_s, theta_cr, n_ed/1e3, n_fi/1e3, m_ed/1e3, m_fi/1e3
            ),
            &format!(
                "θ_eff={:.0} °C (Branddauer {:.0} s), θ_cr={:.0} °C, k_θ={k:.3}, N_Ed={:.1} kN / N_fi,Rd={:.1} kN, M_y,Ed={:.2} kNm / M_fi,Rd={:.2} kNm, u={u:.3}.",
                theta_eff, fire.duration_s, theta_cr, n_ed/1e3, n_fi/1e3, m_ed/1e3, m_fi/1e3
            ),
        ),
        remedies,
    )
}

pub fn check_fatigue(fat: &FatigueDetail, member: Option<&crate::snapshot::AluminiumMember>, annex: AnnexChoice) -> CheckResult {
    let gmf = part_1_3::gamma_mf(annex);
    let dc = part_1_3::detail_category_delta_sigma_c(&fat.detail_category).unwrap_or(fat.delta_sigma_c);
    let member_scale = match member {
        None if !fat.member_id.is_empty() => 0.0,
        Some(m) => {
            let gov = part_en1990::governing_member_effects(m, annex);
            1.0 + gov.m_y_ed.abs() / 1.0e6
        }
        _ => 1.0,
    };
    let delta_ed = if member_scale == 0.0 { fat.delta_sigma_ed } else { fat.delta_sigma_ed * member_scale.max(1e-6) };
    let rd = part_1_3::fatigue_strength_pa(dc, fat.m1, fat.m2, fat.n_cycles) / gmf;
    let damage = part_1_3::damage_ratio(delta_ed, dc, fat.m1, fat.m2, fat.n_cycles, gmf);
    let mut remedies = Vec::new();
    if member.is_none() && !fat.member_id.is_empty() {
        return CheckResult::assess(
            format!("en1999.1-3.fat.{}", fat.id),
            "DIN EN 1999-1-3",
            ClauseId::new("EN 1999-1-3", "§7.1", "7.1"),
            SubjectRef::new(&fat.id, format!("fatigueDetails[id={}].memberId", fat.id), loc("Fatigue", "Ermüdung")),
            loc("Fatigue member reference", "Ermüdungs-Bauteilverweis"),
        )
        .status(crate::document::CheckStatus::Fail)
        .explanation(loc(
            &format!("Fatigue detail '{}' references missing member '{}'.", fat.id, fat.member_id),
            &format!("Ermüdungsdetail '{}' verweist auf fehlendes Bauteil '{}'.", fat.id, fat.member_id),
        ))
        .annex(annex)
        .remedy(Remedy::one_of(
            SubjectRef::new(&fat.id, format!("fatigueDetails[id={}].memberId", fat.id), loc("Fatigue", "Ermüdung")),
            vec![fat.member_id.clone()],
            loc("Point fatigue.memberId at an existing member.", "fatigue.memberId auf vorhandenes Bauteil setzen."),
        ))
        .build();
    }
    if delta_ed > rd && rd > 0.0 {
        remedies.push(Remedy::at_most(
            SubjectRef::new(&fat.id, format!("fatigueDetails[id={}].deltaSigmaEd", fat.id), loc(&format!("Fatigue {}", fat.id), &format!("Ermüdung {}", fat.id))),
            q_stress(delta_ed),
            q_stress(rd),
            loc(
                &format!("Reduce Δσ_Ed from {:.0} MPa to ≤ {:.0} MPa (detail {}).", delta_ed/1e6, rd/1e6, fat.detail_category),
                &format!("Δσ_Ed von {:.0} MPa auf ≤ {:.0} MPa reduzieren (Detail {}).", delta_ed/1e6, rd/1e6, fat.detail_category),
            ),
        ));
        remedies.push(Remedy::one_of(
            SubjectRef::new(&fat.id, format!("fatigueDetails[id={}].detailCategory", fat.id), loc("Detail category", "Kerbfall")),
            vec!["71".into(), "80".into(), "90".into()],
            loc("Select a higher Annex J detail category.", "Höheren Annex-J-Kerbfall wählen."),
        ));
    }
    utilization_check(
        format!("en1999.1-3.fat.{}", fat.id),
        "DIN EN 1999-1-3",
        ClauseId::new("EN 1999-1-3", "§7.1", "7.1"),
        SubjectRef::new(&fat.id, format!("fatigueDetails[id={}].deltaSigmaEd", fat.id), loc(&format!("Fatigue {}", fat.id), &format!("Ermüdung {}", fat.id))),
        loc("Fatigue damage (bi-linear S–N)", "Ermüdungsschädigung (bi-lineare S–N)"),
        q_stress(delta_ed),
        q_stress(rd.max(1e-9)),
        annex,
        loc(
            &format!(
                "Detail {}, Δσ_C={:.0} MPa, m1={:.1}/m2={:.1}, n={:.3e}, γ_Mf={gmf:.2}, D={damage:.3}, member {}.",
                fat.detail_category, dc/1e6, fat.m1, fat.m2, fat.n_cycles, fat.member_id
            ),
            &format!(
                "Detail {}, Δσ_C={:.0} MPa, m1={:.1}/m2={:.1}, n={:.3e}, γ_Mf={gmf:.2}, D={damage:.3}, Bauteil {}.",
                fat.detail_category, dc/1e6, fat.m1, fat.m2, fat.n_cycles, fat.member_id
            ),
        ),
        remedies,
    )
}


fn check_cold_formed(sheet: &crate::snapshot::ColdFormedSheet, material: &AluminiumMaterial, annex: AnnexChoice) -> Vec<CheckResult> {
    let Some(alloy) = part_1_1::resolve_alloy(&material.designation) else {
        return vec![CheckResult::assess(
            format!("en1999.1-4.alloy.{}", sheet.id),
            "DIN EN 1999-1-4",
            ClauseId::new("EN 1999-1-4", "§5", "5.4"),
            SubjectRef::new(&material.id, format!("materials[id={}].designation", material.id), loc("Alloy", "Legierung")),
            loc("Alloy designation", "Legierungsbezeichnung"),
        )
        .status(crate::document::CheckStatus::Fail)
        .explanation(loc(
            &format!("Unknown alloy designation '{}'.", material.designation),
            &format!("Unbekannte Legierungsbezeichnung '{}'.", material.designation),
        ))
        .annex(annex)
        .remedy(Remedy::one_of(
            SubjectRef::new(&material.id, format!("materials[id={}].designation", material.id), loc("Alloy", "Legierung")),
            part_1_1::CATALOGUE_ALLOYS.iter().map(|s| (*s).to_string()).collect(),
            loc("Pick a catalogue alloy from EN 1999-1-1 Table 3.2.", "Kataloglegierung nach EN 1999-1-1 Tabelle 3.2 wählen."),
        ))
        .build()];
    };
    let params = na_de::AnnexParams::for_choice(annex);
    let eps = part_1_1::epsilon(alloy.f_o_pa);
    let beta = if sheet.thickness > 0.0 { sheet.width / sheet.thickness } else { f64::INFINITY };
    let beta_limit = 22.0 * eps;
    let rho = if beta <= beta_limit { 1.0 } else { (beta_limit / beta).min(1.0) };
    let b_eff = rho * sheet.width;
    let wel = b_eff * sheet.thickness.powi(2) / 4.0;
    let m_rd = wel * alloy.f_o_pa / params.gamma_m1;
    let mut out = Vec::new();
    let mut remedies_b = Vec::new();
    if beta > beta_limit {
        let t_req = sheet.width / beta_limit;
        remedies_b.push(Remedy::at_least(
            SubjectRef::new(&sheet.id, format!("coldFormed[id={}].thickness", sheet.id), loc("Sheet thickness", "Blechdicke")),
            q_length(sheet.thickness),
            q_length(t_req),
            loc(
                &format!("Increase thickness so β/ε ≤ {:.1} (t ≥ {:.2} mm).", beta_limit, t_req * 1e3),
                &format!("Dicke erhöhen, damit β/ε ≤ {:.1} (t ≥ {:.2} mm).", beta_limit, t_req * 1e3),
            ),
        ));
    }
    out.push(utilization_check(
        format!("en1999.1-4.local.{}", sheet.id),
        "DIN EN 1999-1-4",
        ClauseId::new("EN 1999-1-4", "§5.4", "5.4"),
        SubjectRef::new(&sheet.id, format!("coldFormed[id={}].thickness", sheet.id), loc(&format!("Sheet {}", sheet.id), &format!("Blech {}", sheet.id))),
        loc("Local buckling of cold-formed sheeting", "Lokales Beulen kaltgeformter Bleche"),
        q_dim(beta / eps),
        q_dim(beta_limit),
        annex,
        loc(
            &format!("b={:.0} mm, t={:.2} mm, β/ε={:.1}, limit={:.1}, ρ={rho:.3}.", sheet.width*1e3, sheet.thickness*1e3, beta/eps, beta_limit),
            &format!("b={:.0} mm, t={:.2} mm, β/ε={:.1}, Grenze={:.1}, ρ={rho:.3}.", sheet.width*1e3, sheet.thickness*1e3, beta/eps, beta_limit),
        ),
        remedies_b,
    ));
    let mut remedies_m = Vec::new();
    if sheet.m_ed.abs() > m_rd && m_rd > 0.0 {
        remedies_m.push(Remedy::at_most(
            SubjectRef::new(&sheet.id, format!("coldFormed[id={}].mEd", sheet.id), loc("Sheet moment", "Blechmoment")),
            q_moment(sheet.m_ed.abs()),
            q_moment(m_rd),
            loc(
                &format!("Reduce M_Ed to ≤ {:.2} kNm or thicken the sheet.", m_rd/1e3),
                &format!("M_Ed auf ≤ {:.2} kNm reduzieren oder Blech verdicken.", m_rd/1e3),
            ),
        ));
    }
    out.push(utilization_check(
        format!("en1999.1-4.bend.{}", sheet.id),
        "DIN EN 1999-1-4",
        ClauseId::new("EN 1999-1-4", "§5.4", "5.4"),
        SubjectRef::new(&sheet.id, format!("coldFormed[id={}].mEd", sheet.id), loc(&format!("Sheet {}", sheet.id), &format!("Blech {}", sheet.id))),
        loc("Sheeting bending resistance", "Biegewiderstand des Blechs"),
        q_moment(sheet.m_ed.abs()),
        q_moment(m_rd),
        annex,
        loc(
            &format!("M_Ed={:.2} kNm, M_c,Rd={:.2} kNm (W_eff={:.0} mm³).", sheet.m_ed.abs()/1e3, m_rd/1e3, wel*1e9),
            &format!("M_Ed={:.2} kNm, M_c,Rd={:.2} kNm (W_eff={:.0} mm³). (DE)", sheet.m_ed.abs()/1e3, m_rd/1e3, wel*1e9),
        ),
        remedies_m,
    ));

    // Axial + bending interaction (EN 1999-1-4 §6.1)
    let a_g = sheet.width * sheet.thickness;
    let n_rd = a_g * alloy.f_o_pa / params.gamma_m1 * (if sheet.welded { alloy.rho_o_haz.min(alloy.rho_u_haz) * part_1_1::eta_factor(true) } else { 1.0 });
    let mut rem_n = Vec::new();
    if sheet.n_ed.abs() > n_rd && n_rd > 0.0 {
        rem_n.push(Remedy::at_most(
            SubjectRef::new(&sheet.id, format!("coldFormed[id={}].nEd", sheet.id), loc("Sheet axial", "Blech-Normalkraft")),
            q_force(sheet.n_ed),
            q_force(n_rd.copysign(sheet.n_ed)),
            loc("Reduce |N_Ed| on the cold-formed sheet.", "|N_Ed| am kaltgeformten Blech reduzieren."),
        ));
    }
    out.push(utilization_check(
        format!("en1999.1-4.axial.{}", sheet.id),
        "DIN EN 1999-1-4",
        ClauseId::new("EN 1999-1-4", "§6.1", "6.1"),
        SubjectRef::new(&sheet.id, format!("coldFormed[id={}].nEd", sheet.id), loc(&format!("Sheet {}", sheet.id), &format!("Blech {}", sheet.id))),
        loc("Cold-formed axial resistance", "Normalkraftwiderstand kaltgeformt"),
        q_force(sheet.n_ed.abs()),
        q_force(n_rd.max(1e-9)),
        annex,
        loc(
            &format!("N_Ed={:.1} kN, welded={}, η/ρ applied, N_Rd={:.1} kN.", sheet.n_ed/1e3, sheet.welded, n_rd/1e3),
            &format!("N_Ed={:.1} kN, geschweißt={}, η/ρ angesetzt, N_Rd={:.1} kN.", sheet.n_ed/1e3, sheet.welded, n_rd/1e3),
        ),
        rem_n,
    ));

    // Web crippling / support reaction from span UDL proxy: R = 2 M_Ed / span
    let r_ed = if sheet.span > 0.0 { 2.0 * sheet.m_ed.abs() / sheet.span } else { 0.0 };
    let r_rd = 0.5 * sheet.thickness * sheet.thickness * alloy.f_o_pa / params.gamma_m1; // simplified
    let mut rem_r = Vec::new();
    if r_ed > r_rd && r_rd > 0.0 {
        rem_r.push(Remedy::at_least(
            SubjectRef::new(&sheet.id, format!("coldFormed[id={}].span", sheet.id), loc("Sheet span", "Blechstützweite")),
            q_length(sheet.span),
            q_length((2.0 * sheet.m_ed.abs() / r_rd).max(sheet.span)),
            loc("Increase span support spacing reduction or thicken sheet.", "Stützweite/Auflager verbessern oder Blech dicken."),
        ));
        rem_r.push(Remedy::at_least(
            SubjectRef::new(&sheet.id, format!("coldFormed[id={}].thickness", sheet.id), loc("Sheet thickness", "Blechdicke")),
            q_length(sheet.thickness),
            q_length((r_ed * params.gamma_m1 / (0.5 * alloy.f_o_pa)).sqrt()),
            loc("Increase thickness against web crippling.", "Dicke gegen Stegkrüppeln erhöhen."),
        ));
    }
    out.push(utilization_check(
        format!("en1999.1-4.support.{}", sheet.id),
        "DIN EN 1999-1-4",
        ClauseId::new("EN 1999-1-4", "§6.1.5", "6.1.5"),
        SubjectRef::new(&sheet.id, format!("coldFormed[id={}].span", sheet.id), loc(&format!("Sheet {}", sheet.id), &format!("Blech {}", sheet.id))),
        loc("Support reaction / web crippling", "Auflagerkraft / Stegkrüppeln"),
        q_force(r_ed),
        q_force(r_rd.max(1e-9)),
        annex,
        loc(
            &format!("Span L={:.2} m → R_Ed={:.2} kN, R_Rd={:.2} kN.", sheet.span, r_ed/1e3, r_rd/1e3),
            &format!("Stützweite L={:.2} m → R_Ed={:.2} kN, R_Rd={:.2} kN.", sheet.span, r_ed/1e3, r_rd/1e3),
        ),
        rem_r,
    ));

    // Interaction N–M
    let u_i = if n_rd > 0.0 && m_rd > 0.0 { sheet.n_ed.abs()/n_rd + sheet.m_ed.abs()/m_rd } else { 0.0 };
    let mut rem_nm = Vec::new();
    if u_i > 1.0 {
        rem_nm.push(Remedy::at_most(
            SubjectRef::new(&sheet.id, format!("coldFormed[id={}].mEd", sheet.id), loc("Sheet moment", "Blechmoment")),
            q_moment(sheet.m_ed.abs()),
            q_moment((m_rd * (1.0 - sheet.n_ed.abs()/n_rd.max(1e-9))).max(0.0)),
            loc("Reduce M_Ed so N–M interaction ≤ 1.", "M_Ed reduzieren, sodass N–M-Interaktion ≤ 1."),
        ));
        rem_nm.push(Remedy::at_most(
            SubjectRef::new(&sheet.id, format!("coldFormed[id={}].nEd", sheet.id), loc("Sheet axial", "Blech-Normalkraft")),
            q_force(sheet.n_ed.abs()),
            q_force((n_rd * (1.0 - sheet.m_ed.abs()/m_rd.max(1e-9))).max(0.0)),
            loc("Reduce |N_Ed| so N–M interaction ≤ 1.", "|N_Ed| reduzieren, sodass N–M-Interaktion ≤ 1."),
        ));
    }
    out.push(utilization_check(
        format!("en1999.1-4.nm.{}", sheet.id),
        "DIN EN 1999-1-4",
        ClauseId::new("EN 1999-1-4", "§6.1.4", "6.1.4"),
        SubjectRef::new(&sheet.id, format!("coldFormed[id={}].mEd", sheet.id), loc(&format!("Sheet {}", sheet.id), &format!("Blech {}", sheet.id))),
        loc("Cold-formed N–M interaction", "N–M-Interaktion kaltgeformt"),
        q_dim(u_i),
        q_dim(1.0),
        annex,
        loc(
            &format!("|N|/N_Rd + |M|/M_Rd = {u_i:.3} (welded={}).", sheet.welded),
            &format!("|N|/N_Rd + |M|/M_Rd = {u_i:.3} (geschweißt={}).", sheet.welded),
        ),
        rem_nm,
    ));
    out
}

fn check_shell(shell: &crate::snapshot::AluminiumShell, material: &AluminiumMaterial, annex: AnnexChoice) -> Vec<CheckResult> {
    let Some(alloy) = part_1_1::resolve_alloy(&material.designation) else {
        return vec![CheckResult::assess(
            format!("en1999.1-5.alloy.{}", shell.id),
            "DIN EN 1999-1-5",
            ClauseId::new("EN 1999-1-5", "§5", "5.3"),
            SubjectRef::new(&material.id, format!("materials[id={}].designation", material.id), loc("Alloy", "Legierung")),
            loc("Alloy designation", "Legierungsbezeichnung"),
        )
        .status(crate::document::CheckStatus::Fail)
        .explanation(loc(
            &format!("Unknown alloy designation '{}'.", material.designation),
            &format!("Unbekannte Legierungsbezeichnung '{}'.", material.designation),
        ))
        .annex(annex)
        .remedy(Remedy::one_of(
            SubjectRef::new(&material.id, format!("materials[id={}].designation", material.id), loc("Alloy", "Legierung")),
            part_1_1::CATALOGUE_ALLOYS.iter().map(|s| (*s).to_string()).collect(),
            loc("Pick a catalogue alloy from EN 1999-1-1 Table 3.2.", "Kataloglegierung nach EN 1999-1-1 Tabelle 3.2 wählen."),
        ))
        .build()];
    };
    let params = na_de::AnnexParams::for_choice(annex);
    let e = na_de::E_PA;
    let r = shell.radius.max(1e-9);
    let t = shell.thickness.max(1e-9);
    let l = shell.length.max(t);
    // EN 1999-1-5 §5.3 / Annex A (C-class fabrication defaults).
    let alpha = 0.62; // C fabrication quality
    let beta_shell = 0.60;
    let eta = 1.0;
    let lambda_0 = 0.20;
    let sigma_x_rcr = 0.605 * e * (t / r);
    let sigma_theta_rcr = 0.92 * e * (t / r) * (r / l).sqrt();
    let tau_rcr = 0.75 * e * (t / r) * (r / l).powf(0.75);
    let lambda_x = if sigma_x_rcr > 0.0 { (alloy.f_o_pa / sigma_x_rcr).sqrt() } else { 0.0 };
    let phi_x = 0.5 * (1.0 + alpha * (lambda_x - lambda_0) + beta_shell * lambda_x.powi(2));
    let chi_x = (1.0 / (phi_x + (phi_x * phi_x - beta_shell * lambda_x.powi(2)).max(0.0).sqrt())).min(1.0) * eta;
    let lambda_th = if sigma_theta_rcr > 0.0 { (alloy.f_o_pa / sigma_theta_rcr).sqrt() } else { 0.0 };
    let phi_th = 0.5 * (1.0 + alpha * (lambda_th - lambda_0) + beta_shell * lambda_th.powi(2));
    let chi_th = (1.0 / (phi_th + (phi_th * phi_th - beta_shell * lambda_th.powi(2)).max(0.0).sqrt())).min(1.0);
    let sigma_x_rd = chi_x * sigma_x_rcr / params.gamma_m1;
    let sigma_theta_rd = chi_th * sigma_theta_rcr.min(alloy.f_o_pa) / params.gamma_m1;
    let _ = tau_rcr;
    let mut out = Vec::new();
    let mut remedies_x = Vec::new();
    if shell.sigma_x_ed.abs() > sigma_x_rd && sigma_x_rd > 0.0 {
        remedies_x.push(Remedy::at_most(
            SubjectRef::new(&shell.id, format!("shells[id={}].sigmaXEd", shell.id), loc("Meridional stress", "Meridianspannung")),
            q_stress(shell.sigma_x_ed.abs()),
            q_stress(sigma_x_rd),
            loc(
                &format!("Reduce σ_x,Ed to ≤ {:.0} MPa or increase t/r.", sigma_x_rd/1e6),
                &format!("σ_x,Ed auf ≤ {:.0} MPa reduzieren oder t/r erhöhen.", sigma_x_rd/1e6),
            ),
        ));
        let t_req = if e > 0.0 && shell.radius > 0.0 {
            shell.sigma_x_ed.abs() * params.gamma_m1 / (chi_x * 0.605 * e) * shell.radius
        } else {
            shell.thickness
        };
        remedies_x.push(Remedy::at_least(
            SubjectRef::new(&shell.id, format!("shells[id={}].thickness", shell.id), loc("Shell thickness", "Schalendicke")),
            q_length(shell.thickness),
            q_length(t_req),
            loc(
                &format!("Increase shell thickness to ≥ {:.2} mm.", t_req*1e3),
                &format!("Schalendicke auf ≥ {:.2} mm erhöhen.", t_req*1e3),
            ),
        ));
    }
    out.push(utilization_check(
        format!("en1999.1-5.buckle.{}", shell.id),
        "DIN EN 1999-1-5",
        ClauseId::new("EN 1999-1-5", "§5.3", "5.3"),
        SubjectRef::new(&shell.id, format!("shells[id={}].sigmaXEd", shell.id), loc(&format!("Shell {}", shell.id), &format!("Schale {}", shell.id))),
        loc("Shell meridional buckling", "Schalenbeulen in Meridianrichtung"),
        q_stress(shell.sigma_x_ed.abs()),
        q_stress(sigma_x_rd),
        annex,
        loc(
            &format!("σ_x,Ed={:.0} MPa, σ_x,Rd={:.0} MPa (σ_x,Rcr={:.0} MPa, χ={chi_x:.2}, r={:.0} mm, t={:.1} mm).", shell.sigma_x_ed.abs()/1e6, sigma_x_rd/1e6, sigma_x_rcr/1e6, shell.radius*1e3, shell.thickness*1e3),
            &format!("σ_x,Ed={:.0} MPa, σ_x,Rd={:.0} MPa (σ_x,Rcr={:.0} MPa, χ={chi_x:.2}, r={:.0} mm, t={:.1} mm). (DE)", shell.sigma_x_ed.abs()/1e6, sigma_x_rd/1e6, sigma_x_rcr/1e6, shell.radius*1e3, shell.thickness*1e3),
        ),
        remedies_x,
    ));
    let mut remedies_t = Vec::new();
    if shell.sigma_theta_ed.abs() > sigma_theta_rd && sigma_theta_rd > 0.0 {
        remedies_t.push(Remedy::at_most(
            SubjectRef::new(&shell.id, format!("shells[id={}].sigmaThetaEd", shell.id), loc("Circumferential stress", "Umfangsspannung")),
            q_stress(shell.sigma_theta_ed.abs()),
            q_stress(sigma_theta_rd),
            loc(
                &format!("Reduce σ_θ,Ed to ≤ {:.0} MPa.", sigma_theta_rd/1e6),
                &format!("σ_θ,Ed auf ≤ {:.0} MPa reduzieren.", sigma_theta_rd/1e6),
            ),
        ));
    }
    out.push(utilization_check(
        format!("en1999.1-5.ring.{}", shell.id),
        "DIN EN 1999-1-5",
        ClauseId::new("EN 1999-1-5", "§5.3", "5.3"),
        SubjectRef::new(&shell.id, format!("shells[id={}].sigmaThetaEd", shell.id), loc(&format!("Shell {}", shell.id), &format!("Schale {}", shell.id))),
        loc("Shell circumferential membrane stress", "Umfangsmembranspannung der Schale"),
        q_stress(shell.sigma_theta_ed.abs()),
        q_stress(sigma_theta_rd),
        annex,
        loc(
            &format!("σ_θ,Ed={:.0} MPa, f_o/γ_M1={:.0} MPa, L={:.0} mm.", shell.sigma_theta_ed.abs()/1e6, sigma_theta_rd/1e6, shell.length*1e3),
            &format!("σ_θ,Ed={:.0} MPa, f_o/γ_M1={:.0} MPa, L={:.0} mm. (DE)", shell.sigma_theta_ed.abs()/1e6, sigma_theta_rd/1e6, shell.length*1e3),
        ),
        remedies_t,
    ));
    out
}


/// � Exhaustive evaluate for the aluminium-structure subject.
pub fn evaluate_structure(doc: &En1999Snapshot) -> CheckReport {
    let mut report = CheckReport::default();
    let annex = doc.annex;

    if doc.members.is_empty() {
        report.push(
            CheckResult::assess(
                "en1999.1-1.empty",
                "DIN EN 1999-1-1",
                ClauseId::new("EN 1999-1-1", "§6", "6"),
                SubjectRef::whole(loc("Structure", "Tragwerk")),
                loc("Member ULS checks", "Bauteil-ULS-Nachweise"),
            )
            .not_applicable(loc("No members in the subject.", "Keine Bauteile im Gegenstand."))
            .annex(annex)
            .build(),
        );
    }

    for member in &doc.members {
        let section = doc.sections.iter().find(|s| s.id == member.section_id);
        let material = doc.materials.iter().find(|m| m.id == member.material_id);
        match (section, material) {
            (Some(sec), Some(mat)) => report.extend(check_member(member, sec, mat, annex)),
            _ => report.push(
                CheckResult::assess(
                    format!("en1999.ref.{}", member.id),
                    "DIN EN 1999-1-1",
                    ClauseId::new("EN 1999-1-1", "§6", "6"),
                    SubjectRef::new(&member.id, format!("members[id={}]", member.id), member_label(&member.id)),
                    loc("Member references", "Bauteil-Referenzen"),
                )
                .status(crate::document::CheckStatus::Fail)
                .explanation(loc("Missing section or material reference.", "Querschnitt- oder Materialverweis fehlt."))
                .annex(annex)
                .remedy(Remedy::one_of(
                    SubjectRef::new(&member.id, format!("members[id={}].sectionId", member.id), member_label(&member.id)),
                    doc.sections.iter().map(|s| s.id.clone()).collect(),
                    loc("Pick an existing section id.", "Vorhandene Querschnitt-Id wählen."),
                ))
                .build(),
            ),
        }
    }

    if doc.connections.is_empty() {
        report.push(
            CheckResult::assess(
                "en1999.8.empty",
                "DIN EN 1999-1-1",
                ClauseId::new("EN 1999-1-1", "§8", "8"),
                SubjectRef::whole(loc("Connections", "Anschlüsse")),
                loc("Connection checks", "Anschlussnachweise"),
            )
            .not_applicable(loc("No connections in the subject.", "Keine Anschlüsse im Gegenstand."))
            .annex(annex)
            .build(),
        );
    }
    for conn in &doc.connections {
        if let Some(mat) = doc.materials.iter().find(|m| m.id == conn.material_id) {
            let mem = doc.members.iter().find(|m| m.id == conn.member_id);
            report.extend(check_connection(conn, mat, mem, annex));
        }
    }

    if doc.fire_scenarios.is_empty() {
        report.push(
            CheckResult::assess(
                "en1999.1-2.empty",
                "DIN EN 1999-1-2",
                ClauseId::new("EN 1999-1-2", "§4", "4"),
                SubjectRef::whole(loc("Fire", "Brand")),
                loc("Fire checks", "Brandschutznachweise"),
            )
            .not_applicable(loc("No fire scenarios claimed.", "Keine Brandszenarien angegeben."))
            .annex(annex)
            .build(),
        );
    }
    for fire in &doc.fire_scenarios {
        let member = doc.members.iter().find(|m| m.id == fire.member_id);
        let section = member.and_then(|m| doc.sections.iter().find(|s| s.id == m.section_id));
        let material = member.and_then(|m| doc.materials.iter().find(|mat| mat.id == m.material_id));
        match (member, section, material) {
            (Some(mem), Some(sec), Some(mat)) => report.push(check_fire(fire, mem, sec, mat, annex)),
            _ => report.push(
                CheckResult::assess(
                    format!("en1999.1-2.fire.{}", fire.id),
                    "DIN EN 1999-1-2",
                    ClauseId::new("EN 1999-1-2", "§4", "4.2"),
                    SubjectRef::new(&fire.id, format!("fireScenarios[id={}].memberId", fire.id), loc("Fire", "Brand")),
                    loc("Fire member reference", "Brand-Bauteilverweis"),
                )
                .status(crate::document::CheckStatus::Fail)
                .explanation(loc(
                    "Fire scenario references a missing member, section or material.",
                    "Brandszenario verweist auf fehlendes Bauteil, Querschnitt oder Material.",
                ))
                .annex(annex)
                .remedy(Remedy::one_of(
                    SubjectRef::new(&fire.id, format!("fireScenarios[id={}].memberId", fire.id), loc("Fire", "Brand")),
                    doc.members.iter().map(|m| m.id.clone()).collect(),
                    loc("Point fire.memberId at an existing member.", "fire.memberId auf vorhandenes Bauteil setzen."),
                ))
                .build(),
            ),
        }
    }

    if doc.fatigue_details.is_empty() {
        report.push(
            CheckResult::assess(
                "en1999.1-3.empty",
                "DIN EN 1999-1-3",
                ClauseId::new("EN 1999-1-3", "§7", "7"),
                SubjectRef::whole(loc("Fatigue", "Ermüdung")),
                loc("Fatigue checks", "Ermüdungsnachweise"),
            )
            .not_applicable(loc("No fatigue details claimed.", "Keine Ermüdungsdetails angegeben."))
            .annex(annex)
            .build(),
        );
    }
    for fat in &doc.fatigue_details {
        let mem = doc.members.iter().find(|m| m.id == fat.member_id);
        report.push(check_fatigue(fat, mem, annex));
    }

    if doc.cold_formed.is_empty() {
        report.push(
            CheckResult::assess(
                "en1999.1-4.empty",
                "DIN EN 1999-1-4",
                ClauseId::new("EN 1999-1-4", "§5", "5.4"),
                SubjectRef::whole(loc("Cold-formed sheeting", "Kaltprofile / Bleche")),
                loc("Cold-formed sheeting checks", "Nachweise für kaltgeformte Bleche"),
            )
            .not_applicable(loc("No cold-formed sheeting in the subject.", "Keine kaltgeformten Bleche im Gegenstand."))
            .annex(annex)
            .build(),
        );
    }
    for sheet in &doc.cold_formed {
        if let Some(mat) = doc.materials.iter().find(|m| m.id == sheet.material_id) {
            report.extend(check_cold_formed(sheet, mat, annex));
        } else {
            report.push(
                CheckResult::assess(
                    format!("en1999.1-4.ref.{}", sheet.id),
                    "DIN EN 1999-1-4",
                    ClauseId::new("EN 1999-1-4", "§5", "5.4"),
                    SubjectRef::new(&sheet.id, format!("coldFormed[id={}].materialId", sheet.id), loc("Sheet", "Blech")),
                    loc("Sheeting material reference", "Blech-Materialverweis"),
                )
                .status(crate::document::CheckStatus::Fail)
                .explanation(loc("Missing material reference for cold-formed sheet.", "Materialverweis für kaltgeformtes Blech fehlt."))
                .annex(annex)
                .remedy(Remedy::one_of(
                    SubjectRef::new(&sheet.id, format!("coldFormed[id={}].materialId", sheet.id), loc("Sheet", "Blech")),
                    doc.materials.iter().map(|m| m.id.clone()).collect(),
                    loc("Point sheet.materialId at an existing material.", "sheet.materialId auf vorhandenes Material setzen."),
                ))
                .build(),
            );
        }
    }

    if doc.shells.is_empty() {
        report.push(
            CheckResult::assess(
                "en1999.1-5.empty",
                "DIN EN 1999-1-5",
                ClauseId::new("EN 1999-1-5", "§5", "5.3"),
                SubjectRef::whole(loc("Shells", "Schalen")),
                loc("Shell buckling checks", "Schalenbeulnachweise"),
            )
            .not_applicable(loc("No shells in the subject.", "Keine Schalen im Gegenstand."))
            .annex(annex)
            .build(),
        );
    }
    for shell in &doc.shells {
        if let Some(mat) = doc.materials.iter().find(|m| m.id == shell.material_id) {
            report.extend(check_shell(shell, mat, annex));
        } else {
            report.push(
                CheckResult::assess(
                    format!("en1999.1-5.ref.{}", shell.id),
                    "DIN EN 1999-1-5",
                    ClauseId::new("EN 1999-1-5", "§5", "5.3"),
                    SubjectRef::new(&shell.id, format!("shells[id={}].materialId", shell.id), loc("Shell", "Schale")),
                    loc("Shell material reference", "Schalen-Materialverweis"),
                )
                .status(crate::document::CheckStatus::Fail)
                .explanation(loc("Missing material reference for shell.", "Materialverweis für Schale fehlt."))
                .annex(annex)
                .remedy(Remedy::one_of(
                    SubjectRef::new(&shell.id, format!("shells[id={}].materialId", shell.id), loc("Shell", "Schale")),
                    doc.materials.iter().map(|m| m.id.clone()).collect(),
                    loc("Point shell.materialId at an existing material.", "shell.materialId auf vorhandenes Material setzen."),
                ))
                .build(),
            );
        }
    }

    report
}

//#endregion 🔖️ComplianceHelpers

//#region 🧪️ComplianceTests
#[cfg(test)]
#[path = "🧪️tests/⚖️compliance/🦀️.rs"]
mod compliance_tests;
//#endregion ️ComplianceTests
