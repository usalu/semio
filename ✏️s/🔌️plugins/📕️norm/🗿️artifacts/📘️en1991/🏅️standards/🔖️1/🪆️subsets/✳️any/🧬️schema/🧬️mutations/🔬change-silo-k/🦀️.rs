//! 🔢 `change-silo-k` — sets the En1991 silo lateral pressure ratio scalar.


use crate::artifacts::en1991::{En1991Diff, En1991Mutation, En1991Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeSiloK {
    pub new_silo_k: f64,
}

impl protocol::MutationKind<En1991Snapshot, En1991Mutation> for ChangeSiloK {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "silo-k", kind: "change-silo-k", record: "ChangedSiloK" };

    fn diff(&self, base: &En1991Snapshot) -> protocol::MutationOutcome<<En1991Mutation as protocol::Mutation<En1991Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1991Snapshot) -> Vec<En1991Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change silo lateral pressure ratio to {:?}", self.new_silo_k)
    }
}
//#endregion 🔖️Payload
