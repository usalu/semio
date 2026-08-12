//! 🧬️ En1990 artifact schema — every field of the artifact with its state class.

use schema::ArtifactSchema;
use crate::artifacts::en1990::En1990QkEntry;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full En1990 artifact state across persistent and shared-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1990")]
pub struct En1990Artifact {
    #[state(persistent)] pub g_k: f64,
    #[state(persistent)] pub q_k: Vec<En1990QkEntry>,
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
