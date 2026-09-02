//! 🧬️ Flow diff schema — sparse field delta over the artifact.

use crate::artifacts::flow::schema::FlowArtifact;
use crate::artifacts::flow::FlowContentChild;
use flow::CameraJson;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔹Diff
/// 🔺️ Sparse field delta for the flow artifact; persistent entries apply via
/// [`MutationDiff`](protocol::MutationDiff). `content` carries a whole-handle replacement (content-
/// addressed, so a changed handle IS the change signal — see `📓️wave3-reports/lowpoly-report.md`'s
/// `mesh: Option<Option<ArtifactChild<…>>>` precedent; flow's `content` slot is never absent, only
/// ever replaced, so a single `Option<FlowContentChild>` — not the double-`Option` an optional slot
/// needs — is the sparse-vs-unchanged signal here, matching writer's `document` field exactly).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.flow.flow")]
pub struct FlowDiff {
    #[state(artifact)]
    pub artifact: Option<Box<FlowArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub camera: Option<CameraJson>,
    #[state(artifact)]
    pub content: Option<FlowContentChild>,
    #[state(presence)]
    pub selected_node_ids: Option<FlowStringList>,
    #[state(presence)]
    pub selected_edge_ids: Option<FlowStringList>,
    #[state(presence)]
    pub selected_handle_ids: Option<FlowStringList>,
    #[state(presence)]
    pub preview_off_node_ids: Option<FlowStringList>,
    #[state(config)]
    pub lod_mode: Option<String>,
    #[state(config)]
    pub proximity_distance: Option<f64>,
    #[state(config)]
    pub grid_visible: Option<bool>,
    #[state(config)]
    pub grid_snap_enabled: Option<bool>,
    #[state(config)]
    pub grid_factor: Option<f64>,
    #[state(config)]
    pub catalogue_sections_json: Option<String>,
    #[state(config)]
    pub automation_enabled_json: Option<String>,
    #[state(config)]
    pub contributions_json: Option<String>,
    #[state(config)]
    pub generation_json: Option<String>,
    #[state(config)]
    pub locale: Option<String>,
}
//#endregion 🔹Diff

//#region 🔹DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct FlowStringList {
    pub values: Vec<String>,
}
//#endregion 🔹DeltaHelpers
