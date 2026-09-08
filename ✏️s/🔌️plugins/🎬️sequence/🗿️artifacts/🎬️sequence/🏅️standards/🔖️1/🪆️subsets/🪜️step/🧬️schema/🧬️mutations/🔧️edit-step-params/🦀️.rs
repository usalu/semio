//! 🩹 Sequence mutation — `EditStepParams`: replaces a step's authored parameter body wholesale
//! (the params dictionary is edited as one blob by the properties panel, never field-by-field).
use crate::diff::SequenceDiff;
use crate::mutations::SequenceMutation;
use crate::schema::operations::{SequenceDetectedMutation, SequenceDetectionContext};
use crate::{SequenceSnapshot, StepParams};

//#region 🔖️Mutation
/// 🩹 `edit-step-params` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
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
            (before.params != step.params).then(|| SequenceDetectedMutation { order: (1, index, 1), mutation: SequenceMutation::EditStepParams(EditStepParams { id: step.id.clone(), params: step.params.clone() }) })
        })
        .collect()
}
//#endregion 🔎️Detection

//#region 🧪️MutationLaws
#[cfg(test)]
#[path = "🧪️tests/🔬️mutation-law/🦀️.rs"]
mod mutation_law_tests;
//#endregion 🧪️MutationLaws
