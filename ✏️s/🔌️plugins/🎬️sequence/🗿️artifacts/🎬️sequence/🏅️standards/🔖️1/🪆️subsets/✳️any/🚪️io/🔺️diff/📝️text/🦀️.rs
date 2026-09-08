//! 🔺️ Sequence artifact — sparse field-delta diff codec and apply/absorb.

use crate::schema::SequenceArtifact;
use crate::SequenceSnapshot;
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::schema::diff::*;

//#region 🔖️Apply
impl SequenceDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
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
            if let Some(value) = &self.last_run_json {
                next.last_run_json = value.clone();
            }
            if let Some(value) = &self.orientation {
                next.orientation = value.clone();
            }
            if let Some(value) = &self.camera {
                next.camera = value.clone();
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
        take!(last_run_json);
        take!(orientation);
        take!(camera);
    }
}
//#endregion 🔖️Apply

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
