//! 🩹 Sequence mutation — `EditStepParams`: replaces a step's authored parameter body wholesale
//! (the params dictionary is edited as one blob by the properties panel, never field-by-field).
use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::mutations::SequenceMutation;
use crate::artifacts::sequence::schema::operations::{SequenceDetectedMutation, SequenceDetectionContext};
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
pub async fn edit_step_params(id: String, params: StepParams) -> SequenceMutation {
    SequenceMutation::EditStepParams(EditStepParams { id, params })
}

impl protocol::MutationKind<SequenceSnapshot, SequenceMutation> for EditStepParams {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "step", kind: "edit-step-params", record: "EditedStep" };

    async fn diff(&self, base: &SequenceSnapshot) -> protocol::MutationOutcome<SequenceDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &SequenceSnapshot) -> Vec<SequenceMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Edit step \"{}\" parameters", self.id)
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
            (before.params != step.params).then(|| SequenceDetectedMutation { order: (1, index, 1), mutation: SequenceMutation::EditStepParams(EditStepParams { id: step.id.clone(), params: step.params.clone() }) })
        })
        .collect()
}
//#endregion 🔎️Detection

//#region 🧪️MutationLaws
#[cfg(test)]
mod mutation_law_tests {
    use super::*;
    use crate::artifacts::sequence::default_snapshot;
    use protocol::testkit::assert_missing_target_is_error;

    #[semio_framework_async_macros::async_test]
    async fn edit_family_missing_target_is_error() {
        let base = default_snapshot();
        assert_missing_target_is_error(&base, &edit_step_params("missing".into(), StepParams::new()));
    }
}
//#endregion 🧪️MutationLaws
