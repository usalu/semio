//! 🌱 Sequence mutation — `CreateStep`: brings a new id-keyed step into existence.
use crate::diff::SequenceDiff;
use crate::mutations::SequenceMutation;
use crate::schema::operations::{SequenceDetectedMutation, SequenceDetectionContext};
use crate::{SequenceSnapshot, SequenceStep};

//#region 🔖️Mutation
/// 🌱 `create-step` payload — full initial payload (position/kind/params/slot all fixed at
/// creation; `slot`/`kind` never change again — `edit-step-params`/`move-step`/
/// `change-step-collapsed` only ever touch `params`/`x`/`y`/`collapsed`).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-step")]
pub struct CreateStep {
    #[dsl(block)]
    pub step: SequenceStep,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_step(step: SequenceStep) -> SequenceMutation {
    SequenceMutation::CreateStep(CreateStep { step })
}

impl protocol::MutationKind<SequenceSnapshot, SequenceMutation> for CreateStep {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "step", kind: "create-step", record: "CreatedStep" };

    fn diff(&self, base: &SequenceSnapshot) -> protocol::MutationOutcome<SequenceDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SequenceSnapshot) -> Vec<SequenceMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create step \"{}\"", self.step.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.step.id.clone()]
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
        .filter(|(_, step)| !context.before_steps.contains_key(step.id.as_str()))
        .map(|(index, step)| SequenceDetectedMutation { order: (1, index, 0), mutation: SequenceMutation::CreateStep(CreateStep { step: step.clone() }) })
        .collect()
}
//#endregion 🔎️Detection

//#region 🧪️MutationLaws
#[cfg(test)]
#[path = "🧪️tests/🔬️mutation-law/🦀️.rs"]
mod mutation_law_tests;
//#endregion 🧪️MutationLaws
