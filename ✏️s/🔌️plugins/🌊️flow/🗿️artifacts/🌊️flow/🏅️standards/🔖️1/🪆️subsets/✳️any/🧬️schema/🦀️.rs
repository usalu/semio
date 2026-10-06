//! 🧬️ Flow artifact schema — every field of the artifact with its state class.

use crate::{FlowContentChild, FlowSnapshot};

use framework_schema::ArtifactSchema;

//#region 🔖️Constants
/// 🖱️ Default proximity-select distance used by each concrete Flow main-window configuration,
/// homed here rather than app-side because this schema's own `FlowArtifact::from_snapshot` needs it too
/// and an artifact must never depend on an app.
pub const FLOW_DEFAULT_PROXIMITY_DISTANCE: f64 = 48.0;
/// 🔳️ Default canvas grid factor — see [`FLOW_DEFAULT_PROXIMITY_DISTANCE`] for why it lives here.
pub const FLOW_DEFAULT_GRID_FACTOR: f64 = 10.0;
//#endregion 🔖️Constants

//#region 🔖️Widgets
/// 🎛️ Every `Widget` variant carries its own `id: String` as its first field — this reaches through
/// the tag to read it generically.
pub fn widget_id(widget: &Widget) -> &str {
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

pub fn widget_kind_label(widget: &Widget) -> &'static str {
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
pub fn widget_with_id(widget: &Widget, id: String) -> Widget {
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

pub fn widget_tree_label(widget: &Widget) -> String {
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
/// 🧬️ flow document artifact state.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.flow.flow")]
pub struct FlowArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub content: FlowContentChild,
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
        FlowSnapshot { schema: self.schema.clone(), content: self.content.clone() }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: FlowSnapshot) -> Self {
        Self { schema: snapshot.schema, content: snapshot.content }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: FlowSnapshot) {
        self.schema = snapshot.schema;
        self.content = snapshot.content;
    }
}
//#endregion 🔹Conversions

//#region 🔹Descriptor
/// 🧬️ Descriptor for `s.flow.flow` — twenty handcrafted schema leaves.
pub fn flow_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.flow.flow",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔹Descriptor
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use semio_framework_artifact_flow_flow::CameraJson;
pub use semio_framework_artifact_flow_flow::Widget;
//#endregion 🔁️Re-exports
