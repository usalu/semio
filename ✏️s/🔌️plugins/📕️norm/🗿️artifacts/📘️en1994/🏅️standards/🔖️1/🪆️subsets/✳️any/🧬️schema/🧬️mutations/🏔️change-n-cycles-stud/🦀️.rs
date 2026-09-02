//! 🔁 `change-n-cycles-stud` — sets the En 1994 stud fatigue cycle count N scalar.


use crate::artifacts::en1994::{En1994Diff, En1994Mutation, En1994Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeNCyclesStud {
    pub new_n_cycles_stud: f64,
}

impl protocol::MutationKind<En1994Snapshot, En1994Mutation> for ChangeNCyclesStud {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "n-cycles-stud", kind: "change-n-cycles-stud", record: "ChangedNCyclesStud" };

    fn diff(&self, base: &En1994Snapshot) -> protocol::MutationOutcome<<En1994Mutation as protocol::Mutation<En1994Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1994Snapshot) -> Vec<En1994Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change fatigue cycle count N to {}", self.new_n_cycles_stud)
    }
}
//#endregion 🔖️Payload
