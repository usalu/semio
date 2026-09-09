//! 🧬️ Flow diff schema — sparse field delta over the artifact.

use crate::FlowContentChild;
use framework_schema::ArtifactSchema;
use semio_framework_artifact_flow_flow::CameraJson;

//#region 🔹Diff
/// 🔺️ Sparse field delta for the flow artifact; persistent entries apply via
/// [`MutationDiff`](protocol::MutationDiff). `content` carries a whole-handle replacement (content-
/// addressed, so a changed handle IS the change signal — see `📓️wave3-reports/lowpoly-report.md`'s
/// `mesh: Option<Option<ArtifactChild<…>>>` precedent; flow's `content` slot is never absent, only
/// ever replaced, so a single `Option<FlowContentChild>` — not the double-`Option` an optional slot
/// needs — is the sparse-vs-unchanged signal here, matching writer's `document` field exactly).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
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
}
//#endregion 🔹Diff

//#region 🔹DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct FlowStringList {
    pub values: Vec<String>,
}
//#endregion 🔹DeltaHelpers

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::schema::FlowArtifact;
//#endregion 🔁️Re-exports
