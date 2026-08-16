//! 🩹 Sequence mutation — `EditStepParams`: replaces a step's authored parameter body wholesale
//! (the params dictionary is edited as one blob by the properties panel, never field-by-field).
use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::mutations::SequenceMutation;
use crate::artifacts::sequence::{SequenceSnapshot, StepParams};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🩹 `edit-step-params` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "edit-step-params")]
pub struct EditStepParams {
    pub id: String,
    pub params: StepParams,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn edit_step_params(id: String, params: StepParams) -> SequenceMutation {
    SequenceMutation::EditStepParams(EditStepParams { id, params })
}

impl protocol::MutationKind<SequenceSnapshot, SequenceMutation> for EditStepParams {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "step", kind: "edit-step-params", record: "EditedStep" };

    fn diff(&self, base: &SequenceSnapshot) -> protocol::MutationOutcome<SequenceDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SequenceSnapshot) -> Vec<SequenceMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Edit step \"{}\" parameters", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
