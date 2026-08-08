//! 🧬️ Flow diff schema — sparse field delta over the artifact.

use crate::artifacts::flow::schema::FlowArtifact;
use flow::{CameraJson, SynapseSpec, Widget, WidgetLayout};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔹Diff
/// 🔺️ Sparse field delta for the flow artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.flow.flow")]
pub struct FlowDiff {
    #[state(persistent)] pub artifact: Option<Box<FlowArtifact>>,
    #[state(persistent)] pub schema: Option<String>,
    #[state(persistent)] pub camera: Option<CameraJson>,
    #[state(persistent)] pub widgets: Option<FlowWidgetsDelta>,
    #[state(persistent)] pub synapses: Option<FlowSynapsesDelta>,
    #[state(persistent)] pub layout: Option<FlowLayoutMapDelta>,
    #[state(shared_ui)] pub selected_node_ids: Option<FlowStringList>,
    #[state(shared_ui)] pub selected_edge_ids: Option<FlowStringList>,
    #[state(shared_ui)] pub selected_handle_ids: Option<FlowStringList>,
    #[state(shared_ui)] pub preview_off_node_ids: Option<FlowStringList>,
    #[state(local_ui)] pub lod_mode: Option<String>,
    #[state(local_ui)] pub proximity_distance: Option<f64>,
    #[state(local_ui)] pub grid_visible: Option<bool>,
    #[state(local_ui)] pub grid_snap_enabled: Option<bool>,
    #[state(local_ui)] pub grid_factor: Option<f64>,
    #[state(local_ui)] pub catalogue_sections_json: Option<String>,
    #[state(local_ui)] pub automation_enabled_json: Option<String>,
    #[state(local_ui)] pub contributions_json: Option<String>,
    #[state(local_ui)] pub generation_json: Option<String>,
    #[state(local_ui)] pub locale: Option<String>,
}
//#endregion 🔹Diff

//#region 🔹DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct FlowStringList {
    pub values: Vec<String>,
}

/// 📂 Layout-map wrapper so optional map diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct FlowLayoutMapDelta {
    pub entries: BTreeMap<String, Option<WidgetLayout>>,
}

/// Identified-collection delta for widgets.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct FlowWidgetsDelta {
    pub added: Vec<Widget>,
    pub removed: Vec<String>,
    pub patched: Vec<FlowWidgetPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched widget entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowWidgetPatchEntry {
    pub id: String,
    pub patch: Widget,
}

/// Identified-collection delta for synapses.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct FlowSynapsesDelta {
    pub added: Vec<SynapseSpec>,
    pub removed: Vec<String>,
    pub patched: Vec<FlowSynapsePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched synapse entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowSynapsePatchEntry {
    pub id: String,
    pub patch: SynapseSpec,
}
//#endregion 🔹DeltaHelpers
