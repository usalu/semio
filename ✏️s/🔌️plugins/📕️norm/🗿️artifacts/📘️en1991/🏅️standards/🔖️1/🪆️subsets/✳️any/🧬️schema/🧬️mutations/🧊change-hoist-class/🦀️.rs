//! ⛓️ `change-hoist-class` — sets the En1991 hoist class scalar.


use crate::artifacts::en1991::{En1991Diff, En1991Mutation, En1991Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeHoistClass {
    pub new_hoist_class: String,
}

impl protocol::MutationKind<En1991Snapshot, En1991Mutation> for ChangeHoistClass {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "hoist-class", kind: "change-hoist-class", record: "ChangedHoistClass" };

    fn diff(&self, base: &En1991Snapshot) -> protocol::MutationOutcome<<En1991Mutation as protocol::Mutation<En1991Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1991Snapshot) -> Vec<En1991Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change hoist class to \"{}\"", self.new_hoist_class)
    }
}
//#endregion 🔖️Payload
