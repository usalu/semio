//! 🧬️ En1994 artifact schema — every field of the artifact with its state class.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full En1994 artifact state across persistent and shared-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1994")]
pub struct En1994Artifact {
    #[state(persistent)] pub annex: crate::document::AnnexChoice,
    #[state(persistent)] pub m_ed_knm: f64,
    #[state(persistent)] pub v_ed_kn: f64,
    #[state(persistent)] pub m_pla: f64,
    #[state(persistent)] pub m_pl_rd: f64,
    #[state(persistent)] pub eta: f64,
    #[state(persistent)] pub v_l_rd: f64,
    #[state(persistent)] pub insulation_thickness_mm: f64,
    #[state(persistent)] pub fire_rating: String,
    #[state(persistent)] pub deck_type: String,
    #[state(persistent)] pub delta_sigma_mpa: f64,
    #[state(persistent)] pub fatigue_detail: String,
    #[state(persistent)] pub d_mm: f64,
    #[state(persistent)] pub h_sc_mm: f64,
    #[state(persistent)] pub f_ck_mpa: f64,
    #[state(persistent)] pub f_u_mpa: f64,
    #[state(persistent)] pub e_cm_mpa: f64,
    #[state(persistent)] pub v_ed_per_stud_kn: f64,
    #[state(persistent)] pub span_m: f64,
    #[state(persistent)] pub f_y_mpa: f64,
    #[state(persistent)] pub n_cycles_stud: f64,
    #[state(persistent)] pub delta_tau_stud_mpa: f64,
    #[state(shared_ui)] pub selected_check_index: Option<u32>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl En1994Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::en1994::En1994Snapshot {
        crate::artifacts::en1994::En1994Snapshot {
            annex: self.annex,
            m_ed_knm: self.m_ed_knm,
            v_ed_kn: self.v_ed_kn,
            m_pla: self.m_pla,
            m_pl_rd: self.m_pl_rd,
            eta: self.eta,
            v_l_rd: self.v_l_rd,
            insulation_thickness_mm: self.insulation_thickness_mm,
            fire_rating: self.fire_rating.clone(),
            deck_type: self.deck_type.clone(),
            delta_sigma_mpa: self.delta_sigma_mpa,
            fatigue_detail: self.fatigue_detail.clone(),
            d_mm: self.d_mm,
            h_sc_mm: self.h_sc_mm,
            f_ck_mpa: self.f_ck_mpa,
            f_u_mpa: self.f_u_mpa,
            e_cm_mpa: self.e_cm_mpa,
            v_ed_per_stud_kn: self.v_ed_per_stud_kn,
            span_m: self.span_m,
            f_y_mpa: self.f_y_mpa,
            n_cycles_stud: self.n_cycles_stud,
            delta_tau_stud_mpa: self.delta_tau_stud_mpa,
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::en1994::En1994Snapshot) -> Self {
        Self {
            annex: snapshot.annex,
            m_ed_knm: snapshot.m_ed_knm,
            v_ed_kn: snapshot.v_ed_kn,
            m_pla: snapshot.m_pla,
            m_pl_rd: snapshot.m_pl_rd,
            eta: snapshot.eta,
            v_l_rd: snapshot.v_l_rd,
            insulation_thickness_mm: snapshot.insulation_thickness_mm,
            fire_rating: snapshot.fire_rating.clone(),
            deck_type: snapshot.deck_type.clone(),
            delta_sigma_mpa: snapshot.delta_sigma_mpa,
            fatigue_detail: snapshot.fatigue_detail.clone(),
            d_mm: snapshot.d_mm,
            h_sc_mm: snapshot.h_sc_mm,
            f_ck_mpa: snapshot.f_ck_mpa,
            f_u_mpa: snapshot.f_u_mpa,
            e_cm_mpa: snapshot.e_cm_mpa,
            v_ed_per_stud_kn: snapshot.v_ed_per_stud_kn,
            span_m: snapshot.span_m,
            f_y_mpa: snapshot.f_y_mpa,
            n_cycles_stud: snapshot.n_cycles_stud,
            delta_tau_stud_mpa: snapshot.delta_tau_stud_mpa,
            selected_check_index: None,
        }
    }
    /// 🔄 Overwrite persistent fields from a snapshot; leave shared-ui untouched.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::en1994::En1994Snapshot) {
        let selected = self.selected_check_index;
        *self = Self::from_snapshot(snapshot);
        self.selected_check_index = selected;
    }
}

//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.en1994` — twenty handcrafted schema leaves.
pub fn en1994_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.norm.en1994",
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
    use crate::artifacts::en1994::{En1994Diff, En1994Mutation, En1994Snapshot};

    #[derive(Clone, Debug, Default)]
    pub struct En1994BuilderConstruction {
        snapshot: En1994Snapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for En1994BuilderConstruction {
        type Snapshot = En1994Snapshot;
        type Mutation = En1994Mutation;
        type Diff = En1994Diff;
        fn empty() -> Self { Self { snapshot: En1994Snapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<En1994Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<En1994Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let d = <En1994Mutation as protocol::Mutation<En1994Snapshot>>::diff(&mutation, &self.snapshot);
            self.snapshot = <En1994Diff as protocol::MutationDiff<En1994Snapshot>>::apply(&d, &self.snapshot);
            (self, d)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <En1994Diff as protocol::MutationDiff<En1994Snapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::en1994::En1994Snapshot;

    #[derive(Clone, Debug, Default)]
    pub struct En1994Parts {
        pub snapshot: Option<En1994Snapshot>,
    }

    pub struct En1994AnalyzerAnalysis;

    impl ArtifactAnalysis for En1994AnalyzerAnalysis {
        type Parts = En1994Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.en1994", standard: StandardId("1"), subset: SubsetId("*") };

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
        construction: derived_construction::En1994BuilderConstruction,
        analysis: derived_analysis::En1994AnalyzerAnalysis,
        composition: super::super::io::derived_composition::En1994ComposerComposition,
    }
    builder: En1994Builder,
    analyzer: En1994Analyzer,
    composer: En1994Composer,
);
//#endregion 🧬️DerivedArtifactFacets
