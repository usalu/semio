//! 🗑️ Sequence mutation — `DeleteStep`: removes an id-keyed step (captures cascade — any edge
//! touching this step is severed too, re-`connect`ed by the inverse).
use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::mutations::SequenceMutation;
use crate::artifacts::sequence::schema::operations::{SequenceDetectedMutation, SequenceDetectionContext};
use crate::artifacts::sequence::SequenceSnapshot;

//#region 🔖️Mutation
/// 🗑️ `delete-step` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "delete-step")]
pub struct DeleteStep {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn delete_step(id: String) -> SequenceMutation {
    SequenceMutation::DeleteStep(DeleteStep { id })
}

impl protocol::MutationKind<SequenceSnapshot, SequenceMutation> for DeleteStep {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "step", kind: "delete-step", record: "DeletedStep" };

    async fn diff(&self, base: &SequenceSnapshot) -> protocol::MutationOutcome<SequenceDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &SequenceSnapshot) -> Vec<SequenceMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete step \"{}\"", self.id)
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
        .before
        .steps
        .iter()
        .enumerate()
        .filter(|(_, step)| !context.after_steps.contains_key(step.id.as_str()))
        .map(|(index, step)| SequenceDetectedMutation { order: (0, index, 0), mutation: SequenceMutation::DeleteStep(DeleteStep { id: step.id.clone() }) })
        .collect()
}
//#endregion 🔎️Detection

//#region 🧪️MutationLaws
#[cfg(test)]
mod mutation_law_tests {
    use super::*;
    use crate::artifacts::sequence::default_snapshot;
    use protocol::testkit::{assert_missing_target_is_error, assert_mutation_inverse_law};

    #[semio_framework_async_macros::async_test]
    async fn delete_step_inverse_law() {
        let base = default_snapshot();
        assert_mutation_inverse_law(&base, &delete_step("step-1".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn delete_family_missing_target_is_error() {
        let base = default_snapshot();
        assert_missing_target_is_error(&base, &delete_step("missing".into()));
    }
}
//#endregion 🧪️MutationLaws
