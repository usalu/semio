//! 🧬️ Flow artifact schema — every field of the artifact with its state class.

use crate::artifacts::flow::FlowSnapshot;
use flow::{CameraJson, SynapseSpec, Widget, WidgetLayout, FLOW_LOD_MODE_AUTOMATIC};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔹Artifact
/// 🧬️ Full flow artifact state across persistent, shared-ui and local-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.flow.flow")]
pub struct FlowArtifact {
    #[state(persistent)] pub schema: String,
    #[state(persistent)] pub camera: CameraJson,
    #[state(persistent)] pub widgets: Vec<Widget>,
    #[state(persistent)] pub synapses: Vec<SynapseSpec>,
    #[state(persistent)] pub layout: BTreeMap<String, WidgetLayout>,
    #[state(shared_ui)] pub selected_node_ids: Vec<String>,
    #[state(shared_ui)] pub selected_edge_ids: Vec<String>,
    #[state(shared_ui)] pub selected_handle_ids: Vec<String>,
    #[state(shared_ui)] pub preview_off_node_ids: Vec<String>,
    #[state(local_ui)] pub lod_mode: String,
    #[state(local_ui)] pub proximity_distance: f64,
    #[state(local_ui)] pub grid_visible: bool,
    #[state(local_ui)] pub grid_snap_enabled: bool,
    #[state(local_ui)] pub grid_factor: f64,
    #[state(local_ui)] pub catalogue_sections_json: String,
    #[state(local_ui)] pub automation_enabled_json: String,
    #[state(local_ui)] pub contributions_json: String,
    #[state(local_ui)] pub generation_json: String,
    #[state(local_ui)] pub locale: String,
}
//#endregion 🔹Artifact

//#region 🔹Conversions
impl Default for FlowArtifact {
    fn default() -> Self {
        Self::from_snapshot(FlowSnapshot::default())
    }
}

impl FlowArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> FlowSnapshot {
        FlowSnapshot {
            schema: self.schema.clone(),
            camera: self.camera.clone(),
            widgets: self.widgets.clone(),
            synapses: self.synapses.clone(),
            layout: self.layout.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: FlowSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            camera: snapshot.camera,
            widgets: snapshot.widgets,
            synapses: snapshot.synapses,
            layout: snapshot.layout,
            selected_node_ids: Vec::new(),
            selected_edge_ids: Vec::new(),
            selected_handle_ids: Vec::new(),
            preview_off_node_ids: Vec::new(),
            lod_mode: FLOW_LOD_MODE_AUTOMATIC.into(),
            proximity_distance: crate::artifacts::flow::engine::FLOW_DEFAULT_PROXIMITY_DISTANCE,
            grid_visible: true,
            grid_snap_enabled: false,
            grid_factor: crate::artifacts::flow::engine::FLOW_DEFAULT_GRID_FACTOR,
            catalogue_sections_json: "[]".into(),
            automation_enabled_json: String::new(),
            contributions_json: "[]".into(),
            generation_json: String::new(),
            locale: "en-US".into(),
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: FlowSnapshot) {
        self.schema = snapshot.schema;
        self.camera = snapshot.camera;
        self.widgets = snapshot.widgets;
        self.synapses = snapshot.synapses;
        self.layout = snapshot.layout;
    }
}
//#endregion 🔹Conversions

//#region 🔹Descriptor
/// 🧬️ Descriptor for `s.flow.flow` — twenty handcrafted schema leaves.
pub fn flow_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.flow.flow",
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
//#endregion 🔹Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use semio_framework_plugin::ArtifactBuilder;
    use crate::artifacts::flow::{FlowDiff, FlowMutation, FlowSnapshot};

    #[derive(Clone, Debug, Default)]
    pub struct FlowBuilderConstruction {
        snapshot: FlowSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for FlowBuilderConstruction {
        type Snapshot = FlowSnapshot;
        type Mutation = FlowMutation;
        type Diff = FlowDiff;
        fn empty() -> Self { Self { snapshot: FlowSnapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<FlowSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<FlowSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            crate::artifacts::flow::schema::mutations::apply_flow_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <FlowDiff as protocol::MutationDiff<FlowSnapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::flow::FlowSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct FlowParts {
        pub snapshot: Option<FlowSnapshot>,
    }

    pub struct FlowAnalyzerAnalysis;

    impl ArtifactAnalysis for FlowAnalyzerAnalysis {
        type Parts = FlowParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.flow", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = FlowParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <FlowSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <FlowSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec FlowBuilderFacets {
        construction: derived_construction::FlowBuilderConstruction,
        analysis: derived_analysis::FlowAnalyzerAnalysis,
        composition: super::super::io::derived_composition::FlowComposerComposition,
    }
    builder: FlowBuilder,
    analyzer: FlowAnalyzer,
    composer: FlowComposer,
);
//#endregion 🧬️DerivedArtifactFacets
