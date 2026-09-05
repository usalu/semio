//! 🏷️ `change-variable-action-category` — sets one `q_k` table entry's `category` label, addressed
//! by BASE-state index.


use crate::artifacts::en1990::{En1990Diff, En1990Mutation, En1990Snapshot, en1990_qk, en1990_qk_child_from_entries};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeVariableActionCategory {
    pub index: usize,
    pub new_category: String,
}

impl protocol::MutationKind<En1990Snapshot, En1990Mutation> for ChangeVariableActionCategory {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "variable-action", kind: "change-variable-action-category", record: "ChangedVariableActionCategory" };

    fn diff(&self, base: &En1990Snapshot) -> protocol::MutationOutcome<<En1990Mutation as protocol::Mutation<En1990Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1990Snapshot) -> Vec<En1990Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change variable action #{} category to \"{}\"", self.index, self.new_category)
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}
//#endregion 🔖️Payload
