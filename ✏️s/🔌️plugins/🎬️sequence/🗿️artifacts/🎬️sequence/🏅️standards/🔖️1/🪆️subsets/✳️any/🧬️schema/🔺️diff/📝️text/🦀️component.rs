//! 🔺️ Sequence artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::sequence::schema::SequenceArtifact;
use crate::artifacts::sequence::SequenceSnapshot;
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::sequence::schema::diff::*;


//#region 🔖️Apply
impl SequenceDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &SequenceArtifact) -> SequenceArtifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
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
        if let Some(value) = &self.locale {
            next.locale = value.clone();
        }
        next
    }
}

impl MutationDiff<SequenceSnapshot> for SequenceDiff {
    fn apply(&self, snapshot: &SequenceSnapshot) -> SequenceSnapshot {
        if let Some(replacement) = &self.artifact {
            return replacement.to_snapshot();
        }
        let mut next = snapshot.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(content) = &self.content {
            next.content = content.clone();
        }
        next
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
        take!(locale);
    }
}
//#endregion 🔖️Apply


//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::sequence::{default_snapshot, SequenceStep, StepParams};
    use protocol::Mutation;

    #[test]
    fn create_step_diff_applies_onto_the_base_snapshot() {
        let base = default_snapshot();
        let step = SequenceStep { id: "step-99".into(), kind: "log.print".into(), params: StepParams::new(), x: 5.0, y: 6.0, slot: None, collapsed: false };
        let operation = crate::artifacts::sequence::mutations::create_step::mutation::create_step(step);
        let diff: SequenceDiff = operation.diff(&base);
        assert!(diff.content.is_some(), "CreateStep must produce a content diff: {diff:?}");
        assert_eq!(diff.apply(&base).to_fixture().steps.len(), base.to_fixture().steps.len() + 1);
    }
}
//#endregion 🧪️Tests
