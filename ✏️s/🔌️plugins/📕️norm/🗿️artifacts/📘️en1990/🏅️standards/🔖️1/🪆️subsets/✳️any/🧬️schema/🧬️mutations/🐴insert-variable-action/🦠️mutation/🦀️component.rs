//! ➕ `insert-variable-action` — places a new variable action (`Q_k` category + characteristic
//! value) at a FINAL-state index in the EN 1990 document's `q_k` table (an intrinsically ordered,
//! anonymous collection — no stable id on `En1990QkEntry`).

use crate::artifacts::en1990::{En1990Mutation, En1990Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InsertVariableAction {
    pub index: usize,
    pub category: String,
    pub value: f64,
}

impl protocol::MutationKind<En1990Snapshot, En1990Mutation> for InsertVariableAction {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "variable-action", kind: "insert-variable-action", record: "InsertedVariableAction" };

    async fn diff(&self, base: &En1990Snapshot) -> protocol::MutationOutcome<<En1990Mutation as protocol::Mutation<En1990Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &En1990Snapshot) -> Vec<En1990Mutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Insert variable action \"{}\" ({}) at #{}", self.category, self.value, self.index)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}
//#endregion 🔖️Payload
