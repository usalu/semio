//! 🧬️ Procedural3d artifact schema — every field of the artifact with its state class.

use crate::artifacts::procedural3d::snapshot::schema::Procedural3dSnapshot;
use flow::CameraJson;
use flow::FlowFixture;
use flow::playbook::GenerationPlayState;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Procedural3dArtifact
/// 🧬️ Procedural3dArtifact facet type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.procedural.procedural3d")]

pub struct Procedural3dArtifact {
    #[state(persistent)] pub fixture: FlowFixture,
    #[state(persistent)] pub generation: GenerationPlayState,
    #[state(shared_ui)] pub selected_node_ids: Vec<String>,
    #[state(local_ui)] pub lod_mode: String,
    #[state(local_ui)] pub show_mode: String,
    #[state(local_ui)] pub selection_method: String,
    #[state(preview)] pub hovered_node_id: Option<String>,
    #[state(local_ui)] pub graph_camera: CameraJson,
    #[state(local_ui)] pub preview_camera: Procedural3dPreviewCamera,
    #[state(local_ui)] pub sun_json: String,
    #[state(shared_ui)] pub selected_generation_id: Option<String>,
    #[state(preview)] pub generation_preview_text: Option<String>,
    #[state(shared_ui)] pub active_utility_id: String,
    #[state(local_ui)] pub locale: String,
    #[state(local_ui)] pub contributions_json: String}
//#endregion 🔖️Procedural3dArtifact

//#region 🔖️PreviewCamera
/// 📷️ 3D preview viewport camera (schema twin of the app config record).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Procedural3dPreviewCamera {
    pub position_x: f64,
    pub position_y: f64,
    pub position_z: f64,
    pub target_x: f64,
    pub target_y: f64,
    pub target_z: f64,
    pub fov: f64}

impl Default for Procedural3dPreviewCamera {
    fn default() -> Self {
        Self {
            position_x: 4.0,
            position_y: -4.0,
            position_z: 3.0,
            target_x: 0.0,
            target_y: 0.0,
            target_z: 0.0,
            fov: 45.0}
    }
}
//#endregion 🔖️PreviewCamera

impl Default for Procedural3dArtifact {
    fn default() -> Self {
        Self {
            fixture: FlowFixture::default(),
            generation: GenerationPlayState::default(),
            selected_node_ids: Vec::new(),
            lod_mode: String::new(),
            show_mode: "shaded".into(),
            selection_method: "rectangle".into(),
            hovered_node_id: None,
            graph_camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
            preview_camera: Procedural3dPreviewCamera::default(),
            sun_json: serde_json::to_string(&semio_framework_plugin::WorldSunConfig::default()).unwrap_or_default(),
            selected_generation_id: None,
            generation_preview_text: None,
            active_utility_id: "move".into(),
            locale: "en-US".into(),
            contributions_json: "[]".into()}
    }
}

impl Procedural3dArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> Procedural3dSnapshot {
        Procedural3dSnapshot {
            fixture: self.fixture.clone(),
            generation: self.generation.clone()}
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: Procedural3dSnapshot) -> Self {
        Self {
            fixture: snapshot.fixture,
            generation: snapshot.generation,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: Procedural3dSnapshot) {
        self.fixture = snapshot.fixture;
        self.generation = snapshot.generation;
    }
}

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.procedural.procedural3d` — twenty handcrafted schema leaves.
pub fn procedural3d_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.procedural.procedural3d",
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
    use crate::artifacts::procedural3d::{Procedural3dDiff, Procedural3dMutation, Procedural3dSnapshot};

    #[derive(Clone, Debug, Default)]
    pub struct Procedural3dBuilderConstruction {
        snapshot: Procedural3dSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for Procedural3dBuilderConstruction {
        type Snapshot = Procedural3dSnapshot;
        type Mutation = Procedural3dMutation;
        type Diff = Procedural3dDiff;
        fn empty() -> Self { Self { snapshot: Procedural3dSnapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<Procedural3dSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Procedural3dSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            crate::artifacts::procedural3d::schema::mutations::apply_procedural3d_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <Procedural3dDiff as protocol::MutationDiff<Procedural3dSnapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::procedural3d::Procedural3dSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct Procedural3dParts {
        pub snapshot: Option<Procedural3dSnapshot>,
    }

    pub struct Procedural3dAnalyzerAnalysis;

    impl ArtifactAnalysis for Procedural3dAnalyzerAnalysis {
        type Parts = Procedural3dParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.procedural3d", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Procedural3dParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <Procedural3dSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Procedural3dSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec Procedural3dBuilderFacets {
        construction: derived_construction::Procedural3dBuilderConstruction,
        analysis: derived_analysis::Procedural3dAnalyzerAnalysis,
        composition: super::super::io::derived_composition::Procedural3dComposerComposition,
    }
    builder: Procedural3dBuilder,
    analyzer: Procedural3dAnalyzer,
    composer: Procedural3dComposer,
);
//#endregion 🧬️DerivedArtifactFacets
