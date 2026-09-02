//! 🏛️ `change-permanent-action` — sets the EN 1990 document's permanent action characteristic
//! value `G_k` (self-weight and other permanent actions, combined per Eq. 6.10/6.10a/6.10b).


use crate::artifacts::en1990::{En1990Diff, En1990Mutation, En1990Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangePermanentAction {
    pub new_g_k: f64,
}

impl protocol::MutationKind<En1990Snapshot, En1990Mutation> for ChangePermanentAction {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "permanent-action", kind: "change-permanent-action", record: "ChangedPermanentAction" };

    fn diff(&self, base: &En1990Snapshot) -> protocol::MutationOutcome<<En1990Mutation as protocol::Mutation<En1990Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1990Snapshot) -> Vec<En1990Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change permanent action G_k to {} kN", self.new_g_k)
    }
}
//#endregion 🔖️Payload
