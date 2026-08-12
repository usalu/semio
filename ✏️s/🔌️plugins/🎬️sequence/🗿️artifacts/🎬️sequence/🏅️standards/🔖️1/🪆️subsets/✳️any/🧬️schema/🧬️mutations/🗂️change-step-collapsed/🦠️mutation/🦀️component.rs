//! 🗂️ Sequence mutation — `ChangeStepCollapsed`: single boolean setter on an addressed step.
use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::mutations::SequenceMutation;
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
pub fn change_step_collapsed(id: String, collapsed: bool) -> SequenceMutation {
    SequenceMutation::ChangeStepCollapsed(ChangeStepCollapsed { id, collapsed })
}

impl protocol::MutationKind<SequenceSnapshot, SequenceMutation> for ChangeStepCollapsed {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "step", kind: "change-step-collapsed", record: "ChangedStepCollapsed" };

    fn diff(&self, base: &SequenceSnapshot) -> SequenceDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SequenceSnapshot) -> Vec<SequenceMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("{} step \"{}\"", if self.collapsed { "Collapse" } else { "Expand" }, self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
