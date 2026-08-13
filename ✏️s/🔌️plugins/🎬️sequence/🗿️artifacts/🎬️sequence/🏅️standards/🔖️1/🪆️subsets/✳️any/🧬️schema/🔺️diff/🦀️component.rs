//! 🧬️ Sequence diff schema — sparse field delta over the artifact.

use crate::artifacts::sequence::{SequenceCamera, SequenceContentChild};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the sequence artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
/// Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`sequence→C:flow`): `steps`/`edges`
/// structured deltas are replaced by a single-`Option<SequenceContentChild>` slot (the composed
/// child is opaque — a parent's diff never embeds a child diff, matching writer's `document` field
/// and flow's `content` field exactly: an always-present slot, never absent, only ever replaced).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.sequence.sequence")]
pub struct SequenceDiff {
    #[state(persistent)]
    pub artifact: Option<Box<crate::artifacts::sequence::schema::SequenceArtifact>>,
    #[state(persistent)]
    pub schema: Option<String>,
    #[state(persistent)]
    pub content: Option<SequenceContentChild>,
    #[state(shared_ui)]
    pub selected_step_ids: Option<SequenceStringList>,
    #[state(local_ui)]
    pub last_run_json: Option<String>,
    #[state(local_ui)]
    pub orientation: Option<String>,
    #[state(local_ui)]
    pub camera: Option<SequenceCamera>,
    #[state(local_ui)]
    pub locale: Option<String>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct SequenceStringList {
    pub values: Vec<String>,
}
//#endregion 🔖️DeltaHelpers
