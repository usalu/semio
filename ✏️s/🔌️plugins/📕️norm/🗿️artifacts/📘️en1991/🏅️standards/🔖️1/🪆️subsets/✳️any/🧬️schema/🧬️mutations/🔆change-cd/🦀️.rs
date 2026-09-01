//! 🌪️ `change-cd` — sets the En1991 dynamic factor c_d scalar.


use crate::artifacts::en1991::{En1991Diff, En1991Mutation, En1991Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeCD {
    pub new_c_d: f64,
}

impl protocol::MutationKind<En1991Snapshot, En1991Mutation> for ChangeCD {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "cd", kind: "change-cd", record: "ChangedCd" };

    fn diff(&self, base: &En1991Snapshot) -> protocol::MutationOutcome<<En1991Mutation as protocol::Mutation<En1991Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1991Snapshot) -> Vec<En1991Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change dynamic factor c_d to {:?}", self.new_c_d)
    }
}
//#endregion 🔖️Payload
