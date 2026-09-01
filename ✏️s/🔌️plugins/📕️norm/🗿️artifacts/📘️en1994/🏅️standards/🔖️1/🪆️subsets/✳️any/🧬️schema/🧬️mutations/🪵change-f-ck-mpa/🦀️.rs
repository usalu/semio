//! 🧊 `change-f-ck-mpa` — sets the En 1994 concrete characteristic cylinder strength f_ck [MPa] scalar.


use crate::artifacts::en1994::{En1994Diff, En1994Mutation, En1994Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeFCkMpa {
    pub new_f_ck_mpa: f64,
}

impl protocol::MutationKind<En1994Snapshot, En1994Mutation> for ChangeFCkMpa {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "f-ck-mpa", kind: "change-f-ck-mpa", record: "ChangedFCkMpa" };

    fn diff(&self, base: &En1994Snapshot) -> protocol::MutationOutcome<<En1994Mutation as protocol::Mutation<En1994Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1994Snapshot) -> Vec<En1994Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change concrete strength f_ck to {}", self.new_f_ck_mpa)
    }
}
//#endregion 🔖️Payload
