//! 🦠️ `🔢change-generation-value` payload and its `MutationKind` impl; diff/inverse delegate to the sibling leaves.
use crate::artifacts::procedural2d::diff::Procedural2dDiff;
use crate::artifacts::procedural2d::mutations::Procedural2dMutation;
use crate::artifacts::procedural2d::{widget_id, Procedural2dSnapshot};
use flow::playbook::FormGeneration;
use flow::Widget;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeGenerationValue {
    pub id: String,
    pub question_id: String,
    pub value: serde_json::Value,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_generation_value(id: String, question_id: String, value: serde_json::Value) -> Procedural2dMutation {
    Procedural2dMutation::ChangeGenerationValue(ChangeGenerationValue { id, question_id, value })
}

impl MutationKind<Procedural2dSnapshot, Procedural2dMutation> for ChangeGenerationValue {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "generation-value", kind: "change-generation-value", record: "ChangedGenerationValue" };

    fn diff(&self, base: &Procedural2dSnapshot) -> protocol::MutationOutcome<Procedural2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Procedural2dSnapshot) -> Vec<Procedural2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change generation \"{}\" value \"{}\"", self.id, self.question_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone(), self.question_id.clone()]
    }
}
//#endregion 🔖️Mutation
