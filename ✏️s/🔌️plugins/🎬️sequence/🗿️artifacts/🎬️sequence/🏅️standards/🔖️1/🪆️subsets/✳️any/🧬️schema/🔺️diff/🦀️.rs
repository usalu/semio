//! 🧬️ Sequence diff schema — sparse field delta over the artifact.

use crate::SequenceContentChild;
use framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the sequence artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
/// Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`sequence→C:flow`): `steps`/`edges`
/// structured deltas are replaced by a single-`Option<SequenceContentChild>` slot (the composed
/// child is opaque — a parent's diff never embeds a child diff, matching writer's `document` field
/// and flow's `content` field exactly: an always-present slot, never absent, only ever replaced).
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord)]
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

use crate::schema::SequenceArtifact;
use crate::SequenceSnapshot;
use protocol::MutationDiff;

impl SequenceDiff {
    /// 🧬️ Applies sparse document fields onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &SequenceArtifact) -> protocol::MutationApplyResult<SequenceArtifact> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
            }
            let mut next = artifact.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            if let Some(content) = &self.content {
                next.content = content.clone();
            }
            next
        })
    }
}

impl MutationDiff<SequenceSnapshot> for SequenceDiff {
    fn apply(&self, snapshot: &SequenceSnapshot) -> protocol::MutationApplyResult<SequenceSnapshot> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
            }
            let mut next = snapshot.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            if let Some(content) = &self.content {
                next.content = content.clone();
            }
            next
        })
    }
    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(schema);
        take!(content);
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
