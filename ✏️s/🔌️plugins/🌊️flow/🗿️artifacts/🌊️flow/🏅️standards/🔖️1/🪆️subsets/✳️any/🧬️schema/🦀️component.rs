//! 🧬️ Flow artifact schema — every field of the artifact with its state class.

use crate::artifacts::flow::{FlowContentChild, FlowSnapshot};
use flow::{CameraJson, Widget, FLOW_LOD_MODE_AUTOMATIC};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Constants
/// 🖱️ Default proximity-select distance — also `FlowConfig`'s own default (`crate::editor::flow::config`),
/// homed here rather than app-side because this schema's own `FlowArtifact::from_snapshot` needs it too
/// and an artifact must never depend on an app.
pub const FLOW_DEFAULT_PROXIMITY_DISTANCE: f64 = 48.0;
/// 🔳️ Default canvas grid factor — see [`FLOW_DEFAULT_PROXIMITY_DISTANCE`] for why it lives here.
pub const FLOW_DEFAULT_GRID_FACTOR: f64 = 10.0;
//#endregion 🔖️Constants

//#region 🔖️Widgets
/// 🎛️ Every `Widget` variant carries its own `id: String` as its first field — this reaches through
/// the tag to read it generically.
pub async fn widget_id(widget: &Widget) -> &str {
    match widget {
        Widget::Neuron { id, .. }
        | Widget::InputSlider { id, .. }
        | Widget::InputNote { id, .. }
        | Widget::InputImage { id, .. }
        | Widget::Variable { id, .. }
        | Widget::OutputPreview { id, .. }
        | Widget::OutputAction { id, .. }
        | Widget::OutputExport { id, .. }
        | Widget::Cluster { id, .. } => id,
    }
}

pub async fn widget_kind_label(widget: &Widget) -> &'static str {
    match widget {
        Widget::Neuron { .. } => "neuron",
        Widget::InputSlider { .. } => "inputSlider",
        Widget::InputNote { .. } => "inputNote",
        Widget::InputImage { .. } => "inputImage",
        Widget::Variable { .. } => "variable",
        Widget::OutputPreview { .. } => "outputPreview",
        Widget::OutputAction { .. } => "outputAction",
        Widget::OutputExport { .. } => "outputExport",
        Widget::Cluster { .. } => "cluster",
    }
}

/// 👯️ Clones a widget with every field but `id` copied verbatim — the `duplicate-widget` composite
/// mutation's plan uses this to mint the copy it hands to `create-widget`.
pub async fn widget_with_id(widget: &Widget, id: String) -> Widget {
    let mut copy = widget.clone();
    match &mut copy {
        Widget::Neuron { id: widget_id, .. }
        | Widget::InputSlider { id: widget_id, .. }
        | Widget::InputNote { id: widget_id, .. }
        | Widget::InputImage { id: widget_id, .. }
        | Widget::Variable { id: widget_id, .. }
        | Widget::OutputPreview { id: widget_id, .. }
        | Widget::OutputAction { id: widget_id, .. }
        | Widget::OutputExport { id: widget_id, .. }
        | Widget::Cluster { id: widget_id, .. } => *widget_id = id,
    }
    copy
}

pub async fn widget_tree_label(widget: &Widget) -> String {
    match widget {
        Widget::Neuron { id, neuron_kind, .. } => format!("{id} ({neuron_kind})"),
        Widget::InputSlider { id, .. } => format!("{id} (slider)"),
        Widget::InputNote { id, .. } => format!("{id} (note)"),
        Widget::OutputPreview { id, .. } => format!("{id} (preview)"),
        Widget::Variable { id, name, .. } => format!("{id} ({name})"),
        widget => format!("{} ({})", widget_id(widget), widget_kind_label(widget)),
    }
}
//#endregion 🔖️Widgets

//#region 🔹Artifact
/// 🧬️ Full flow artifact state across the artifact, presence and config lanes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.flow.flow")]
pub struct FlowArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub camera: CameraJson,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.flow")]
    pub content: FlowContentChild,
    #[state(presence)]
    pub selected_node_ids: Vec<String>,
    #[state(presence)]
    pub selected_edge_ids: Vec<String>,
    #[state(presence)]
    pub selected_handle_ids: Vec<String>,
    #[state(presence)]
    pub preview_off_node_ids: Vec<String>,
    #[state(config)]
    pub lod_mode: String,
    #[state(config)]
    pub proximity_distance: f64,
    #[state(config)]
    pub grid_visible: bool,
    #[state(config)]
    pub grid_snap_enabled: bool,
    #[state(config)]
    pub grid_factor: f64,
    #[state(config)]
    pub catalogue_sections_json: String,
    #[state(config)]
    pub automation_enabled_json: String,
    #[state(config)]
    pub contributions_json: String,
    #[state(config)]
    pub generation_json: String,
    #[state(config)]
    pub locale: String,
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
    pub async fn to_snapshot(&self) -> FlowSnapshot {
        FlowSnapshot { schema: self.schema.clone(), camera: self.camera.clone(), content: self.content.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub async fn from_snapshot(snapshot: FlowSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            camera: snapshot.camera,
            content: snapshot.content,
            selected_node_ids: Vec::new(),
            selected_edge_ids: Vec::new(),
            selected_handle_ids: Vec::new(),
            preview_off_node_ids: Vec::new(),
            lod_mode: FLOW_LOD_MODE_AUTOMATIC.into(),
            proximity_distance: FLOW_DEFAULT_PROXIMITY_DISTANCE,
            grid_visible: true,
            grid_snap_enabled: false,
            grid_factor: FLOW_DEFAULT_GRID_FACTOR,
            catalogue_sections_json: "[]".into(),
            automation_enabled_json: String::new(),
            contributions_json: "[]".into(),
            generation_json: String::new(),
            locale: "en-US".into(),
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub async fn set_snapshot(&mut self, snapshot: FlowSnapshot) {
        self.schema = snapshot.schema;
        self.camera = snapshot.camera;
        self.content = snapshot.content;
    }
}
//#endregion 🔹Conversions

//#region 🔹Descriptor
/// 🧬️ Descriptor for `s.flow.flow` — twenty handcrafted schema leaves.
pub async fn flow_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
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
    use crate::artifacts::flow::{FlowDiff, FlowMutation, FlowSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct FlowBuilderConstruction {
        snapshot: FlowSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for FlowBuilderConstruction {
        type Snapshot = FlowSnapshot;
        type Mutation = FlowMutation;
        type Diff = FlowDiff;
        async fn empty() -> Self {
            Self { snapshot: FlowSnapshot::default(), diagnostics: Vec::new() }
        }
        async fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        async fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<FlowSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        async fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<FlowSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        async fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        async fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <FlowDiff as protocol::MutationDiff<FlowSnapshot>>::apply(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
            Ok(self)
        }
        async fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
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
    use crate::artifacts::flow::FlowSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct FlowParts {
        pub snapshot: Option<FlowSnapshot>,
    }

    pub struct FlowAnalyzerAnalysis;

    impl ArtifactAnalysis for FlowAnalyzerAnalysis {
        type Parts = FlowParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.flow", standard: StandardId("1"), subset: SubsetId("*") };

        async fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        async fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
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
        construction: FlowBuilderConstruction,
        analysis: FlowAnalyzerAnalysis,
        composition: super::super::io::derived_composition::FlowComposerComposition,
    }
    builder: FlowBuilder,
    analyzer: FlowAnalyzer,
    composer: FlowComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn widget_id_and_kind_label_agree_across_variants() {
        let widget = Widget::InputSlider { id: "slider".into(), value: 3.0, min: 0.0, max: 10.0, step: 0.1 };
        assert_eq!(widget_id(&widget), "slider");
        assert_eq!(widget_kind_label(&widget), "inputSlider");
        assert_eq!(widget_tree_label(&widget), "slider (slider)");
    }
}
//#endregion 🧪️Tests
