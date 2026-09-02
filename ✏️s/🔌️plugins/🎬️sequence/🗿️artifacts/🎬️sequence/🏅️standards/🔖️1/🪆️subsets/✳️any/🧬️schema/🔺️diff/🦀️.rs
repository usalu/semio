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
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifacts::sequence::schema::SequenceArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub content: Option<SequenceContentChild>,
    #[state(config)]
    pub last_run_json: Option<String>,
    #[state(config)]
    pub orientation: Option<String>,
    #[state(config)]
    pub camera: Option<SequenceCamera>,
    #[state(config)]
    pub locale: Option<String>,
}
//#endregion 🔖️Diff
