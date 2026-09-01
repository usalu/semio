//! ➕ `insert-variable-action` — places a new variable action (`Q_k` category + characteristic
//! value) at a FINAL-state index in the EN 1990 document's `q_k` table (an intrinsically ordered,
//! anonymous collection — no stable id on `En1990QkEntry`).


use crate::artifacts::en1990::{En1990Diff, En1990Mutation, En1990QkEntry, En1990Snapshot, en1990_qk, en1990_qk_child_from_entries};
use crate::artifacts::en1990::mutations::remove_variable_action;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertVariableAction {
    pub index: usize,
    pub category: String,
    pub value: f64,
}

impl protocol::MutationKind<En1990Snapshot, En1990Mutation> for InsertVariableAction {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "variable-action", kind: "insert-variable-action", record: "InsertedVariableAction" };

    fn diff(&self, base: &En1990Snapshot) -> protocol::MutationOutcome<<En1990Mutation as protocol::Mutation<En1990Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1990Snapshot) -> Vec<En1990Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Insert variable action \"{}\" ({}) at #{}", self.category, self.value, self.index)
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}
//#endregion 🔖️Payload
