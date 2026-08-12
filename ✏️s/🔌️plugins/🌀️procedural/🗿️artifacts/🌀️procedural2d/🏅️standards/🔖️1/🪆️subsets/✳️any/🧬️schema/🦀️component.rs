//! 🧬️ Procedural2d artifact schema — every field of the artifact with its state class.

use crate::artifacts::procedural2d::snapshot::schema::Procedural2dSnapshot;
use flow::CameraJson;
use flow::FlowFixture;
use flow::playbook::GenerationPlayState;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Procedural2dArtifact
/// 🧬️ Procedural2dArtifact facet type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.procedural.procedural2d")]

pub struct Procedural2dArtifact {
    #[state(persistent)] pub fixture: FlowFixture,
    #[state(persistent)] pub generation: GenerationPlayState,
    #[state(shared_ui)] pub selected_ids: Vec<String>,
    #[state(local_ui)] pub graph_camera: CameraJson,
    #[state(local_ui)] pub show_mode: String,
    #[state(shared_ui)] pub selected_generation_id: Option<String>,
    #[state(preview)] pub generation_preview_text: Option<String>,
    #[state(local_ui)] pub locale: String}
//#endregion 🔖️Procedural2dArtifact

impl Default for Procedural2dArtifact {
    fn default() -> Self {
        Self {
            fixture: FlowFixture::default(),
            generation: GenerationPlayState::default(),
            selected_ids: Vec::new(),
            graph_camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
            show_mode: "preview".into(),
            selected_generation_id: None,
            generation_preview_text: None,
            locale: "en-US".into()}
    }
}

impl Procedural2dArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> Procedural2dSnapshot {
        Procedural2dSnapshot {
            fixture: self.fixture.clone(),
            generation: self.generation.clone()}
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: Procedural2dSnapshot) -> Self {
        Self {
            fixture: snapshot.fixture,
            generation: snapshot.generation,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: Procedural2dSnapshot) {
        self.fixture = snapshot.fixture;
        self.generation = snapshot.generation;
    }
}

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.procedural.procedural2d` — twenty handcrafted schema leaves.
pub fn procedural2d_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.procedural.procedural2d",
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
    use crate::artifacts::procedural2d::{Procedural2dDiff, Procedural2dMutation, Procedural2dSnapshot};

    #[derive(Clone, Debug, Default)]
    pub struct Procedural2dBuilderConstruction {
        snapshot: Procedural2dSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for Procedural2dBuilderConstruction {
        type Snapshot = Procedural2dSnapshot;
        type Mutation = Procedural2dMutation;
        type Diff = Procedural2dDiff;
        fn empty() -> Self { Self { snapshot: Procedural2dSnapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<Procedural2dSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Procedural2dSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            crate::artifacts::procedural2d::schema::mutations::apply_procedural2d_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <Procedural2dDiff as protocol::MutationDiff<Procedural2dSnapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::procedural2d::Procedural2dSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct Procedural2dParts {
        pub snapshot: Option<Procedural2dSnapshot>,
    }

    pub struct Procedural2dAnalyzerAnalysis;

    impl ArtifactAnalysis for Procedural2dAnalyzerAnalysis {
        type Parts = Procedural2dParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.procedural2d", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Procedural2dParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <Procedural2dSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Procedural2dSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec Procedural2dBuilderFacets {
        construction: derived_construction::Procedural2dBuilderConstruction,
        analysis: derived_analysis::Procedural2dAnalyzerAnalysis,
        composition: super::super::io::derived_composition::Procedural2dComposerComposition,
    }
    builder: Procedural2dBuilder,
    analyzer: Procedural2dAnalyzer,
    composer: Procedural2dComposer,
);
//#endregion 🧬️DerivedArtifactFacets
