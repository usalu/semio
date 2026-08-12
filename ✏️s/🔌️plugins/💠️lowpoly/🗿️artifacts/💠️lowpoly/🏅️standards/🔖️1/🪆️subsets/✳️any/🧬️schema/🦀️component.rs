//! 🧬️ Lowpoly artifact schema — every field of the artifact with its state class.

use crate::artifacts::lowpoly::{LowpolyObject, LowpolySelection};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full lowpoly artifact state across persistent, shared-ui, local-ui and preview classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.lowpoly.lowpoly")]
pub struct LowpolyArtifact {
    #[state(persistent)] pub schema: String,
    #[state(persistent)] pub objects: Vec<LowpolyObject>,
    #[state(shared_ui)] pub active_object_id: Option<String>,
    #[state(shared_ui)] pub selection: LowpolySelection,
    #[state(shared_ui)] pub selected_object_ids: Vec<String>,
    #[state(shared_ui)] pub paint_utility: String,
    #[state(shared_ui)] pub active_paint_layer: u32,
    #[state(shared_ui)] pub active_utility_id: String,
    #[state(local_ui)] pub show_edges: bool,
    #[state(local_ui)] pub sun_enabled: bool,
    #[state(local_ui)] pub sun_azimuth: f64,
    #[state(local_ui)] pub sun_elevation: f64,
    #[state(local_ui)] pub sun_intensity: f64,
    #[state(local_ui)] pub sun_color: String,
    #[state(local_ui)] pub world_camera_position_x: f64,
    #[state(local_ui)] pub world_camera_position_y: f64,
    #[state(local_ui)] pub world_camera_position_z: f64,
    #[state(local_ui)] pub world_camera_target_x: f64,
    #[state(local_ui)] pub world_camera_target_y: f64,
    #[state(local_ui)] pub world_camera_target_z: f64,
    #[state(local_ui)] pub world_camera_fov: f64,
    #[state(local_ui)] pub utility_params_json: String,
    #[state(local_ui)] pub paint_color_r: u32,
    #[state(local_ui)] pub paint_color_g: u32,
    #[state(local_ui)] pub paint_color_b: u32,
    #[state(local_ui)] pub paint_color_a: u32,
    #[state(local_ui)] pub selection_method: String,
    #[state(local_ui)] pub selection_mode_default: String,
    #[state(local_ui)] pub engagement_input: String,
    #[state(local_ui)] pub locale: String,
    #[state(preview)] pub hovered_object_id: Option<String>,
    #[state(preview)] pub hovered_target_object_id: Option<String>,
    #[state(preview)] pub hovered_target_mode: Option<String>,
    #[state(preview)] pub hovered_target_id: Option<u32>,
    #[state(preview)] pub stroke_drag_active: bool,
    #[state(preview)] pub transform_drag_active: bool,
    #[state(preview)] pub preview_seq: i64,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for LowpolyArtifact {
    fn default() -> Self {
        Self {
            schema: crate::artifacts::lowpoly::LOWPOLY_DOCUMENT_SCHEMA.into(),
            objects: Vec::new(),
            active_object_id: None,
            selection: crate::artifacts::lowpoly::LowpolySelection::default(),
            selected_object_ids: Vec::new(),
            paint_utility: "brush".into(),
            active_paint_layer: 0,
            active_utility_id: "move".into(),
            show_edges: true,
            sun_enabled: false,
            sun_azimuth: 45.0,
            sun_elevation: 35.0,
            sun_intensity: 0.85,
            sun_color: "#ffffff".into(),
            world_camera_position_x: 18.0,
            world_camera_position_y: -18.0,
            world_camera_position_z: 12.0,
            world_camera_target_x: 0.0,
            world_camera_target_y: 0.0,
            world_camera_target_z: 0.0,
            world_camera_fov: 45.0,
            utility_params_json: String::new(),
            paint_color_r: 255,
            paint_color_g: 64,
            paint_color_b: 64,
            paint_color_a: 255,
            selection_method: "rectangle".into(),
            selection_mode_default: "default".into(),
            engagement_input: String::new(),
            locale: "en-US".into(),
            hovered_object_id: None,
            hovered_target_object_id: None,
            hovered_target_mode: None,
            hovered_target_id: None,
            stroke_drag_active: false,
            transform_drag_active: false,
            preview_seq: 0,
        }
    }
}

impl LowpolyArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::lowpoly::LowpolySnapshot {
        crate::artifacts::lowpoly::LowpolySnapshot {
            schema: self.schema.clone(),
            objects: self.objects.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::lowpoly::LowpolySnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            objects: snapshot.objects,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::lowpoly::LowpolySnapshot) {
        self.schema = snapshot.schema;
        self.objects = snapshot.objects;
    }
}
//#endregion 🔖️Conversions


//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.lowpoly.lowpoly` — twenty handcrafted schema leaves.
pub fn lowpoly_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.lowpoly.lowpoly",
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
    use crate::artifacts::lowpoly::schema::diff::LowpolyDiff;
    use crate::artifacts::lowpoly::schema::mutations::LowpolyMutation;
    use crate::artifacts::lowpoly::schema::snapshot::LowpolySnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct LowpolyBuilderConstruction {
        snapshot: LowpolySnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for LowpolyBuilderConstruction {
        type Snapshot = LowpolySnapshot;
        type Mutation = LowpolyMutation;
        type Diff = LowpolyDiff;
        fn empty() -> Self { Self { snapshot: LowpolySnapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<LowpolySnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<LowpolySnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let d = <LowpolyMutation as protocol::Mutation<LowpolySnapshot>>::diff(&mutation, &self.snapshot);
            self.snapshot = protocol::MutationDiff::apply(&d, &self.snapshot);
            (self, d)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <LowpolyDiff as protocol::MutationDiff<LowpolySnapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::lowpoly::LowpolySnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct LowpolyParts {
        pub snapshot: Option<LowpolySnapshot>,
    }

    pub struct LowpolyAnalyzerAnalysis;

    impl ArtifactAnalysis for LowpolyAnalyzerAnalysis {
        type Parts = LowpolyParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.lowpoly", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = LowpolyParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <LowpolySnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <LowpolySnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec LowpolyBuilderFacets {
        construction: derived_construction::LowpolyBuilderConstruction,
        analysis: derived_analysis::LowpolyAnalyzerAnalysis,
        composition: super::super::io::derived_composition::LowpolyComposerComposition,
    }
    builder: LowpolyBuilder,
    analyzer: LowpolyAnalyzer,
    composer: LowpolyComposer,
);
//#endregion 🧬️DerivedArtifactFacets
