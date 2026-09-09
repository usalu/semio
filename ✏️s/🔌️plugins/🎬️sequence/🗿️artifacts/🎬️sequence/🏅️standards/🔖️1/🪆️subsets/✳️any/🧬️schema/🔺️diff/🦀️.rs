//! 🧬️ Sequence diff schema — sparse field delta over the artifact.

use crate::SequenceContentChild;
use framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the sequence artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
/// Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`sequence→C:flow`): `steps`/`edges`
/// structured deltas are replaced by a single-`Option<SequenceContentChild>` slot (the composed
/// child is opaque — a parent's diff never embeds a child diff, matching writer's `document` field
/// and flow's `content` field exactly: an always-present slot, never absent, only ever replaced).
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.sequence.sequence")]
pub struct SequenceDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::schema::SequenceArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub content: Option<SequenceContentChild>,
}
//#endregion 🔖️Diff
