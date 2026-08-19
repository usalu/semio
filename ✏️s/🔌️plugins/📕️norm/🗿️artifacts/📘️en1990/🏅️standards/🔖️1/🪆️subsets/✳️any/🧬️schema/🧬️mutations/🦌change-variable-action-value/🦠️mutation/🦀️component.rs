//! 🔢 `change-variable-action-value` — sets one `q_k` table entry's characteristic value, addressed
//! by BASE-state index.

use crate::artifacts::en1990::{En1990Mutation, En1990Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeVariableActionValue {
    pub index: usize,
    pub new_value: f64,
}

impl protocol::MutationKind<En1990Snapshot, En1990Mutation> for ChangeVariableActionValue {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "variable-action", kind: "change-variable-action-value", record: "ChangedVariableActionValue" };

    async fn diff(&self, base: &En1990Snapshot) -> protocol::MutationOutcome<<En1990Mutation as protocol::Mutation<En1990Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &En1990Snapshot) -> Vec<En1990Mutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change variable action #{} value to {}", self.index, self.new_value)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}
//#endregion 🔖️Payload
