//! 🌱 Sequence mutation — `CreateStep`: brings a new id-keyed step into existence.
use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::mutations::SequenceMutation;
use crate::artifacts::sequence::{SequenceSnapshot, SequenceStep};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🌱 `create-step` payload — full initial payload (position/kind/params/slot all fixed at
/// creation; `slot`/`kind` never change again per `SequenceStepPatch`'s doc comment).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
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

    fn diff(&self, base: &SequenceSnapshot) -> SequenceDiff {
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
