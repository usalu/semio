//! 🧯 `change-fire-member-capacity-c` — sets the En1991 fire member capacity factor scalar.


use crate::artifacts::en1991::{En1991Diff, En1991Mutation, En1991Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeFireMemberCapacityC {
    pub new_fire_member_capacity_c: f64,
}

impl protocol::MutationKind<En1991Snapshot, En1991Mutation> for ChangeFireMemberCapacityC {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fire-member-capacity-c", kind: "change-fire-member-capacity-c", record: "ChangedFireMemberCapacityC" };

    fn diff(&self, base: &En1991Snapshot) -> protocol::MutationOutcome<<En1991Mutation as protocol::Mutation<En1991Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1991Snapshot) -> Vec<En1991Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change fire member capacity factor to {:?}", self.new_fire_member_capacity_c)
    }
}
//#endregion 🔖️Payload
