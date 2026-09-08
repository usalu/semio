//! 🗑️ Sequence mutation — `DeleteStep`: removes an id-keyed step (captures cascade — any edge
//! touching this step is severed too, re-`connect`ed by the inverse).
use crate::diff::SequenceDiff;
use crate::mutations::SequenceMutation;
use crate::schema::operations::{SequenceDetectedMutation, SequenceDetectionContext};
use crate::SequenceSnapshot;

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
pub fn delete_step(id: String) -> SequenceMutation {
    SequenceMutation::DeleteStep(DeleteStep { id })
}

impl protocol::MutationKind<SequenceSnapshot, SequenceMutation> for DeleteStep {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "step", kind: "delete-step", record: "DeletedStep" };

    fn diff(&self, base: &SequenceSnapshot) -> protocol::MutationOutcome<SequenceDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SequenceSnapshot) -> Vec<SequenceMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Delete step \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
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
#[path = "🧪️tests/🔬️mutation-law/🦀️.rs"]
mod mutation_law_tests;
//#endregion 🧪️MutationLaws
