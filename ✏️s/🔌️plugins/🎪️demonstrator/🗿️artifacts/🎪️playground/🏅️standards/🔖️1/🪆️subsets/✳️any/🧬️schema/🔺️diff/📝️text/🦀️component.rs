//! 🔺️ Playground artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::playground::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot;
use crate::artifacts::playground::standards::v1::subsets::any::schema::PlaygroundArtifact;
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::playground::standards::v1::subsets::any::schema::diff::*;

//#region 🔖️Apply
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
//#endregion 🔖️Apply

//#region 🔖️Helpers
/// 🖼️ Whole-snapshot replacement diff.
pub fn diff_set_snapshot(snapshot: &PlaygroundSnapshot) -> PlaygroundDiff {
    PlaygroundDiff { artifact: Some(Box::new(PlaygroundArtifact::from_snapshot(snapshot.clone()))), ..Default::default() }
}
//#endregion 🔖️Helpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_diff_is_a_no_operation() {
        let base = crate::artifacts::playground::standards::v1::subsets::any::schema::empty_playground_snapshot();
        let diff = PlaygroundDiff::default();
        assert_eq!(diff.apply(&base).expect("valid mutation diff"), base);
    }
}
//#endregion 🧪️Tests
