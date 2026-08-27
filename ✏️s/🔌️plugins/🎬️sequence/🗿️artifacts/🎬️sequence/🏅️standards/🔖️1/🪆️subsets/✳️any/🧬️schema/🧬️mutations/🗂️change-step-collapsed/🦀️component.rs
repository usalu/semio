//! 🗂️ Sequence mutation — `ChangeStepCollapsed`: single boolean setter on an addressed step.
use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::mutations::SequenceMutation;
use crate::artifacts::sequence::schema::operations::{SequenceDetectedMutation, SequenceDetectionContext};
use crate::artifacts::sequence::SequenceSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🗂️ `change-step-collapsed` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-step-collapsed")]
pub struct ChangeStepCollapsed {
    pub id: String,
    pub collapsed: bool,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_step_collapsed(id: String, collapsed: bool) -> SequenceMutation {
    SequenceMutation::ChangeStepCollapsed(ChangeStepCollapsed { id, collapsed })
}

impl protocol::MutationKind<SequenceSnapshot, SequenceMutation> for ChangeStepCollapsed {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "step", kind: "change-step-collapsed", record: "ChangedStepCollapsed" };

    async fn diff(&self, base: &SequenceSnapshot) -> protocol::MutationOutcome<SequenceDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &SequenceSnapshot) -> Vec<SequenceMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("{} step \"{}\"", if self.collapsed { "Collapse" } else { "Expand" }, self.id)
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
            (before.collapsed != step.collapsed).then(|| SequenceDetectedMutation { order: (1, index, 2), mutation: SequenceMutation::ChangeStepCollapsed(ChangeStepCollapsed { id: step.id.clone(), collapsed: step.collapsed }) })
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
    async fn change_family_missing_target_is_error() {
        let base = default_snapshot();
        assert_missing_target_is_error(&base, &change_step_collapsed("missing".into(), true));
    }
}
//#endregion 🧪️MutationLaws
