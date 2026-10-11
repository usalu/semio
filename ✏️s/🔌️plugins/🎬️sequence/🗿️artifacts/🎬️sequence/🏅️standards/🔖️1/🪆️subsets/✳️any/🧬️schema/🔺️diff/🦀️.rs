//! 🧬️ Sequence diff schema — sparse field delta over the artifact.

use crate::SequenceContentChild;
use framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the sequence artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
/// Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`sequence→C:flow`): `steps`/`edges`
/// structured deltas are replaced by a single-`Option<SequenceContentChild>` slot (the composed
/// child is opaque — a parent's diff never embeds a child diff, matching writer's `document` field
/// and flow's `content` field exactly: an always-present slot, never absent, only ever replaced).
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.sequence.sequence")]
pub struct SequenceDiff {
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub content: Option<SequenceContentChild>,
}
//#endregion 🔖️Diff

use crate::SequenceSnapshot;
use protocol::{ApplyCapability, DiffAlgebra, MutationApplyResult, MutationDiff};

impl MutationDiff<SequenceSnapshot> for SequenceDiff {
    fn apply(&self, snapshot: &SequenceSnapshot, _capability: ApplyCapability) -> MutationApplyResult<SequenceSnapshot> {
        Ok(SequenceSnapshot { schema: self.schema.clone().unwrap_or_else(|| snapshot.schema.clone()), content: self.content.clone().unwrap_or_else(|| snapshot.content.clone()) })
    }
    fn absorb(&mut self, other: Self) {
        if other.schema.is_some() {
            self.schema = other.schema;
        }
        if other.content.is_some() {
            self.content = other.content;
        }
    }
}

impl DiffAlgebra<SequenceSnapshot> for SequenceDiff {
    fn inverse(&self, base: &SequenceSnapshot) -> Self {
        Self { schema: self.schema.as_ref().map(|_| base.schema.clone()), content: self.content.as_ref().map(|_| base.content.clone()) }
    }
    fn is_empty(&self) -> bool {
        self.schema.is_none() && self.content.is_none()
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
