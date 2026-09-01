//! 🔗 `change-eta` — sets the En 1994 degree of shear connection η scalar.


use crate::artifacts::en1994::{En1994Diff, En1994Mutation, En1994Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeEta {
    pub new_eta: f64,
}

impl protocol::MutationKind<En1994Snapshot, En1994Mutation> for ChangeEta {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "eta", kind: "change-eta", record: "ChangedEta" };

    fn diff(&self, base: &En1994Snapshot) -> protocol::MutationOutcome<<En1994Mutation as protocol::Mutation<En1994Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1994Snapshot) -> Vec<En1994Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change shear connection degree η to {}", self.new_eta)
    }
}
//#endregion 🔖️Payload
