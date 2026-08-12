//! 🧬️ Shooting artifact schema — every field of the artifact with its state class.

use crate::artifacts::shooting::{
    ShootingAsset, ShootingCamera, ShootingSavedCamera, ShootingSceneLighting, ShootingShot,
};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full shooting artifact state across persistent, shared-ui, local-ui and preview classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.shooting.shooting")]
pub struct ShootingArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub assets: Vec<ShootingAsset>,
    #[state(persistent)]
    pub saved_cameras: Vec<ShootingSavedCamera>,
    #[state(persistent)]
    pub scene: ShootingSceneLighting,
    #[state(persistent)]
    pub shots: Vec<ShootingShot>,
    #[state(persistent)]
    pub active_shot_id: String,
    #[state(persistent)]
    pub active_asset_id: String,
    #[state(shared_ui)]
    pub selected_shot_ids: Vec<String>,
    #[state(shared_ui)]
    pub selected_asset_ids: Vec<String>,
    #[state(shared_ui)]
    pub active_utility_id: String,
    #[state(local_ui)]
    pub default_shot_format: String,
    #[state(local_ui)]
    pub default_shot_shape: String,
    #[state(local_ui)]
    pub default_asset_format: String,
    #[state(local_ui)]
    pub selection_method: String,
    #[state(local_ui)]
    pub center_model: bool,
    #[state(local_ui)]
    pub fit_revision: u32,
    #[state(local_ui)]
    pub camera_draft_label: String,
    #[state(local_ui)]
    pub camera: ShootingCamera,
    #[state(local_ui)]
    pub locale: String,
    #[state(preview)]
    pub hovered_asset_id: Option<String>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for ShootingArtifact {
    fn default() -> Self {
        Self {
            schema: crate::artifacts::shooting::SHOOTING_DOCUMENT_SCHEMA.into(),
            assets: Vec::new(),
            saved_cameras: Vec::new(),
            scene: ShootingSceneLighting::default(),
            shots: Vec::new(),
            active_shot_id: String::new(),
            active_asset_id: String::new(),
            selected_shot_ids: Vec::new(),
            selected_asset_ids: Vec::new(),
            active_utility_id: "move".into(),
            default_shot_format: "png".into(),
            default_shot_shape: "rectangle".into(),
            default_asset_format: "glb".into(),
            selection_method: "rectangle".into(),
            center_model: true,
            fit_revision: 0,
            camera_draft_label: String::new(),
            camera: ShootingCamera::default(),
            locale: "en-US".into(),
            hovered_asset_id: None,
        }
    }
}

impl ShootingArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::shooting::ShootingSnapshot {
        crate::artifacts::shooting::ShootingSnapshot {
            schema: self.schema.clone(),
            assets: self.assets.clone(),
            saved_cameras: self.saved_cameras.clone(),
            scene: self.scene.clone(),
            shots: self.shots.clone(),
            active_shot_id: self.active_shot_id.clone(),
            active_asset_id: self.active_asset_id.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::shooting::ShootingSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            assets: snapshot.assets,
            saved_cameras: snapshot.saved_cameras,
            scene: snapshot.scene,
            shots: snapshot.shots,
            active_shot_id: snapshot.active_shot_id,
            active_asset_id: snapshot.active_asset_id,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::shooting::ShootingSnapshot) {
        self.schema = snapshot.schema;
        self.assets = snapshot.assets;
        self.saved_cameras = snapshot.saved_cameras;
        self.scene = snapshot.scene;
        self.shots = snapshot.shots;
        self.active_shot_id = snapshot.active_shot_id;
        self.active_asset_id = snapshot.active_asset_id;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.shooting.shooting` — twenty handcrafted schema leaves.
pub fn shooting_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.shooting.shooting",
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
    use crate::artifacts::shooting::schema::diff::ShootingDiff;
    use crate::artifacts::shooting::schema::mutations::ShootingMutation;
    use crate::artifacts::shooting::schema::snapshot::ShootingSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct ShootingBuilderConstruction {
        snapshot: ShootingSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for ShootingBuilderConstruction {
        type Snapshot = ShootingSnapshot;
        type Mutation = ShootingMutation;
        type Diff = ShootingDiff;
        fn empty() -> Self { Self { snapshot: ShootingSnapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<ShootingSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<ShootingSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let d = <ShootingMutation as protocol::Mutation<ShootingSnapshot>>::diff(&mutation, &self.snapshot);
            self.snapshot = protocol::MutationDiff::apply(&d, &self.snapshot);
            (self, d)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <ShootingDiff as protocol::MutationDiff<ShootingSnapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::shooting::ShootingSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct ShootingParts {
        pub snapshot: Option<ShootingSnapshot>,
    }

    pub struct ShootingAnalyzerAnalysis;

    impl ArtifactAnalysis for ShootingAnalyzerAnalysis {
        type Parts = ShootingParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.shooting", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = ShootingParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <ShootingSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <ShootingSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec ShootingBuilderFacets {
        construction: derived_construction::ShootingBuilderConstruction,
        analysis: derived_analysis::ShootingAnalyzerAnalysis,
        composition: super::super::io::derived_composition::ShootingComposerComposition,
    }
    builder: ShootingBuilder,
    analyzer: ShootingAnalyzer,
    composer: ShootingComposer,
);
//#endregion 🧬️DerivedArtifactFacets
