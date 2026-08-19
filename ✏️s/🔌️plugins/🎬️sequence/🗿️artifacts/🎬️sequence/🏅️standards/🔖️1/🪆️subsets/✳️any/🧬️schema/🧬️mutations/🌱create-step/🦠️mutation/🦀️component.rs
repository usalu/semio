//! 🌱 Sequence mutation — `CreateStep`: brings a new id-keyed step into existence.
use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::mutations::SequenceMutation;
use crate::artifacts::sequence::{SequenceSnapshot, SequenceStep};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🌱 `create-step` payload — full initial payload (position/kind/params/slot all fixed at
/// creation; `slot`/`kind` never change again — `edit-step-params`/`move-step`/
/// `change-step-collapsed` only ever touch `params`/`x`/`y`/`collapsed`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
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
