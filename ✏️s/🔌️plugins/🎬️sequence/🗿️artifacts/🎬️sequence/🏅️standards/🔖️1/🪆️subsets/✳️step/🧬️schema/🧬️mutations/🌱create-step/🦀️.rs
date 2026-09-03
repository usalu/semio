//! 🌱 Sequence mutation — `CreateStep`: brings a new id-keyed step into existence.
use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::mutations::SequenceMutation;
use crate::artifacts::sequence::schema::operations::{SequenceDetectedMutation, SequenceDetectionContext};
use crate::artifacts::sequence::{SequenceSnapshot, SequenceStep};

//#region 🔖️Mutation
/// 🌱 `create-step` payload — full initial payload (position/kind/params/slot all fixed at
/// creation; `slot`/`kind` never change again — `edit-step-params`/`move-step`/
/// `change-step-collapsed` only ever touch `params`/`x`/`y`/`collapsed`).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "create-step")]
pub struct CreateStep {
    #[dsl(block)]
    pub step: SequenceStep,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn create_step(step: SequenceStep) -> SequenceMutation {
    SequenceMutation::CreateStep(CreateStep { step })
}

impl protocol::MutationKind<SequenceSnapshot, SequenceMutation> for CreateStep {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "step", kind: "create-step", record: "CreatedStep" };

    async fn diff(&self, base: &SequenceSnapshot) -> protocol::MutationOutcome<SequenceDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &SequenceSnapshot) -> Vec<SequenceMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create step \"{}\"", self.step.id)
    }
    async fn target(&self) -> Vec<String> {
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
mod mutation_law_tests {
    use super::*;
    use crate::artifacts::sequence::{default_snapshot, StepParams};
    use protocol::{
        testkit::{assert_fatal_never_applies, assert_mutation_inverse_law},
        Mutation,
    };

    #[semio_framework_async_macros::async_test]
    async fn create_step_inverse_law() {
        let base = default_snapshot();
        let step = SequenceStep { id: "step-99".into(), kind: "log.print".into(), params: StepParams::new(), x: 5.0, y: 6.0, slot: None, collapsed: false };
        assert_mutation_inverse_law(&base, &create_step(step));
    }

    #[semio_framework_async_macros::async_test]
    async fn create_family_fatal_never_applies() {
        let base = default_snapshot();
        let outcome = create_step(SequenceStep { id: "step-1".into(), kind: "log.print".into(), params: StepParams::new(), x: 0.0, y: 0.0, slot: None, collapsed: false }).diff(&base);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
        assert_fatal_never_applies(&outcome);
    }
}
//#endregion 🧪️MutationLaws
