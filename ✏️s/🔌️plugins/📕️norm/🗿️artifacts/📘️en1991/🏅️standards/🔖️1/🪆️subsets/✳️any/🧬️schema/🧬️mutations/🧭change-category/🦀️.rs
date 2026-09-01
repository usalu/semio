//! 🏷️ `change-category` — sets the En1991 imposed category scalar.


use crate::artifacts::en1991::{En1991Diff, En1991Mutation, En1991Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeCategory {
    pub new_category: crate::document::ImposedCategory,
}

impl protocol::MutationKind<En1991Snapshot, En1991Mutation> for ChangeCategory {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "category", kind: "change-category", record: "ChangedCategory" };

    fn diff(&self, base: &En1991Snapshot) -> protocol::MutationOutcome<<En1991Mutation as protocol::Mutation<En1991Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1991Snapshot) -> Vec<En1991Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change imposed category to {:?}", self.new_category)
    }
}
//#endregion 🔖️Payload
