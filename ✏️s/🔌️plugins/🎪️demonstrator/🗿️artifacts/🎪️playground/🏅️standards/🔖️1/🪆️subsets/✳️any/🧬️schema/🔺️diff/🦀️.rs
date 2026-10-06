//! 🧬️ Playground diff schema — sparse field delta over the artifact.

use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the playground artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.demonstrator.playground")]
pub struct PlaygroundDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::standards::v1::subsets::any::schema::PlaygroundArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
}
//#endregion 🔖️Diff

use crate::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot;
use crate::standards::v1::subsets::any::schema::PlaygroundArtifact;
use protocol::MutationDiff;

impl PlaygroundDiff {
    /// 🧬️ Applies every sparse entry onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &PlaygroundArtifact) -> protocol::MutationApplyResult<PlaygroundArtifact> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
            }
            let mut next = artifact.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            next
        })
    }
}

impl MutationDiff<PlaygroundSnapshot> for PlaygroundDiff {
    fn apply(&self, snapshot: &PlaygroundSnapshot) -> protocol::MutationApplyResult<PlaygroundSnapshot> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
            }
            let mut next = snapshot.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            next
        })
    }
    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        if other.schema.is_some() {
            self.schema = other.schema;
        }
    }
}

/// 🖼️ Whole-snapshot replacement diff.
pub fn diff_set_snapshot(snapshot: &PlaygroundSnapshot) -> PlaygroundDiff {
    PlaygroundDiff { artifact: Some(Box::new(PlaygroundArtifact::from_snapshot(snapshot.clone()))), ..Default::default() }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
