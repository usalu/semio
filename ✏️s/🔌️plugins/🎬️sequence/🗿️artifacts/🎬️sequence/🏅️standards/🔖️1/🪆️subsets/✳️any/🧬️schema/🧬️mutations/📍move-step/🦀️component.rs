//! ↔️ Sequence mutation — `MoveStep`: absolute spatial reposition of a canvas node.
use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::mutations::SequenceMutation;
use crate::artifacts::sequence::schema::operations::{SequenceDetectedMutation, SequenceDetectionContext};
use crate::artifacts::sequence::SequenceSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ↔️ `move-step` payload — FINAL-state absolute `(x, y)`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "move-step")]
pub struct MoveStep {
    pub id: String,
    pub x: f64,
    pub y: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn move_step(id: String, x: f64, y: f64) -> SequenceMutation {
    SequenceMutation::MoveStep(MoveStep { id, x, y })
}

impl protocol::MutationKind<SequenceSnapshot, SequenceMutation> for MoveStep {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "step", kind: "move-step", record: "MovedStep" };

    async fn diff(&self, base: &SequenceSnapshot) -> protocol::MutationOutcome<SequenceDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &SequenceSnapshot) -> Vec<SequenceMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Move step \"{}\" to ({}, {})", self.id, self.x, self.y)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

//#region 🔎️Detection
/// 🔎️ Detects this leaf's contribution to a before/after sequence plan.
pub fn detect(context: &SequenceDetectionContext<'_>) -> Vec<SequenceDetectedMutation> {
    context
        .after
        .steps
        .iter()
        .enumerate()
        .filter_map(|(index, step)| {
            let before = context.before_steps.get(step.id.as_str())?;
            (before.x != step.x || before.y != step.y).then(|| SequenceDetectedMutation { order: (1, index, 0), mutation: SequenceMutation::MoveStep(MoveStep { id: step.id.clone(), x: step.x, y: step.y }) })
        })
        .collect()
}
//#endregion 🔎️Detection

//#region 🧪️MutationLaws
#[cfg(test)]
mod mutation_law_tests {
    use super::*;
    use crate::artifacts::sequence::default_snapshot;
    use protocol::{
        testkit::{assert_fatal_never_applies, assert_missing_target_is_error, assert_mutation_diff_absorb_law, assert_mutation_inverse_law},
        Mutation,
    };

    #[semio_framework_async_macros::async_test]
    async fn move_step_inverse_law() {
        let base = default_snapshot();
        assert_mutation_inverse_law(&base, &move_step("step-1".into(), 42.0, -8.0));
    }

    #[semio_framework_async_macros::async_test]
    async fn move_step_diff_absorb_law() {
        use protocol::Mutation;
        let base = default_snapshot();
        let d1 = move_step("step-1".into(), 10.0, 10.0).diff(&base).into_parts().0;
        let mid = protocol::MutationDiff::apply(&d1, &base).expect("valid mutation diff");
        let d2 = move_step("step-1".into(), 20.0, 30.0).diff(&mid).into_parts().0;
        assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[semio_framework_async_macros::async_test]
    async fn move_family_missing_target_is_error() {
        let base = default_snapshot();
        assert_missing_target_is_error(&base, &move_step("missing".into(), 1.0, 1.0));
    }

    #[semio_framework_async_macros::async_test]
    async fn move_family_fatal_never_applies() {
        let base = default_snapshot();
        let outcome = move_step("step-1".into(), f64::NAN, 0.0).diff(&base);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
        assert_fatal_never_applies(&outcome);
    }
}
//#endregion 🧪️MutationLaws
