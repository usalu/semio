//! 💪 `change-fu-mpa` — sets the En 1994 shear stud ultimate tensile strength f_u [MPa] scalar.


use crate::artifacts::en1994::{En1994Diff, En1994Mutation, En1994Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeFUMpa {
    pub new_f_u_mpa: f64,
}

impl protocol::MutationKind<En1994Snapshot, En1994Mutation> for ChangeFUMpa {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fu-mpa", kind: "change-fu-mpa", record: "ChangedFUMpa" };

    fn diff(&self, base: &En1994Snapshot) -> protocol::MutationOutcome<<En1994Mutation as protocol::Mutation<En1994Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1994Snapshot) -> Vec<En1994Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change stud ultimate strength f_u to {}", self.new_f_u_mpa)
    }
}
//#endregion 🔖️Payload
