//! 🔀 `reorder-variable-actions` — repositions one variable action within the `q_k` table order
//! (never spatial — `En1990QkEntry` carries no position of its own, only table sequence).

use crate::artifacts::en1990::{En1990Mutation, En1990Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReorderVariableActions {
    pub from: usize,
    pub to: usize,
}

impl protocol::MutationKind<En1990Snapshot, En1990Mutation> for ReorderVariableActions {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "variable-action", kind: "reorder-variable-actions", record: "ReorderedVariableActions" };

    fn diff(&self, base: &En1990Snapshot) -> <En1990Mutation as protocol::Mutation<En1990Snapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1990Snapshot) -> Vec<En1990Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Move variable action #{} to #{}", self.from, self.to)
    }
    fn target(&self) -> Vec<String> {
        vec![self.from.to_string()]
    }
}
//#endregion 🔖️Payload
