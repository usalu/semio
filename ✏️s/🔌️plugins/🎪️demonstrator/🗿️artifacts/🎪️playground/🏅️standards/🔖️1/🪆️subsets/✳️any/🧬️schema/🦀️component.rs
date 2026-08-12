//! 🧬️ Playground artifact schema — every field of the artifact with its state class.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full playground artifact state (persistent fields only today).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.demonstrator.playground")]
pub struct PlaygroundArtifact {
    #[state(persistent)]
    pub schema: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for PlaygroundArtifact {
    fn default() -> Self {
        Self {
            schema: crate::artifacts::playground::PLAYGROUND_DOCUMENT_SCHEMA.into(),
        }
    }
}

impl PlaygroundArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::playground::PlaygroundSnapshot {
        crate::artifacts::playground::PlaygroundSnapshot {
            schema: self.schema.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    pub fn from_snapshot(snapshot: crate::artifacts::playground::PlaygroundSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::playground::PlaygroundSnapshot) {
        self.schema = snapshot.schema;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.demonstrator.playground` — twenty handcrafted schema leaves.
pub fn playground_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.demonstrator.playground",
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
    use crate::artifacts::playground::schema::diff::PlaygroundDiff;
    use crate::artifacts::playground::schema::mutations::PlaygroundMutation;
    use crate::artifacts::playground::schema::snapshot::PlaygroundSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct PlaygroundBuilderConstruction {
        snapshot: PlaygroundSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for PlaygroundBuilderConstruction {
        type Snapshot = PlaygroundSnapshot;
        type Mutation = PlaygroundMutation;
        type Diff = PlaygroundDiff;
        fn empty() -> Self { Self { snapshot: PlaygroundSnapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<PlaygroundSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<PlaygroundSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let d = <PlaygroundMutation as protocol::Mutation<PlaygroundSnapshot>>::diff(&mutation, &self.snapshot);
            self.snapshot = protocol::MutationDiff::apply(&d, &self.snapshot);
            (self, d)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <PlaygroundDiff as protocol::MutationDiff<PlaygroundSnapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::playground::PlaygroundSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct PlaygroundParts {
        pub snapshot: Option<PlaygroundSnapshot>,
    }

    pub struct PlaygroundAnalyzerAnalysis;

    impl ArtifactAnalysis for PlaygroundAnalyzerAnalysis {
        type Parts = PlaygroundParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.playground", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = PlaygroundParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <PlaygroundSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <PlaygroundSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec PlaygroundBuilderFacets {
        construction: derived_construction::PlaygroundBuilderConstruction,
        analysis: derived_analysis::PlaygroundAnalyzerAnalysis,
        composition: super::super::io::derived_composition::PlaygroundComposerComposition,
    }
    builder: PlaygroundBuilder,
    analyzer: PlaygroundAnalyzer,
    composer: PlaygroundComposer,
);
//#endregion 🧬️DerivedArtifactFacets
