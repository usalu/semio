//! ➖ `remove-variable-action` — takes a variable action out of the EN 1990 document's `q_k` table
//! by BASE-state index.

use crate::artifacts::en1990::{En1990Mutation, En1990Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoveVariableAction {
    pub index: usize,
}

impl protocol::MutationKind<En1990Snapshot, En1990Mutation> for RemoveVariableAction {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "variable-action", kind: "remove-variable-action", record: "RemovedVariableAction" };

    fn diff(&self, base: &En1990Snapshot) -> protocol::MutationOutcome<<En1990Mutation as protocol::Mutation<En1990Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1990Snapshot) -> Vec<En1990Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Remove variable action #{}", self.index)
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}
//#endregion 🔖️Payload
