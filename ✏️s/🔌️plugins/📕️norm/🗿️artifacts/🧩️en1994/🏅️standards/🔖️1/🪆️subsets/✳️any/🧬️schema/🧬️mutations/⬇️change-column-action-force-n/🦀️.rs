//! `change-column-action-force-n` mutation leaf.

use crate::{En1994Mutation, En1994Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeColumnActionForceN {
    pub index: usize,
    pub action_index: usize,
    pub new_n_k_n: f64,
}

impl protocol::MutationKind<En1994Snapshot, En1994Mutation> for ChangeColumnActionForceN {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "column-action-force-n", kind: "change-column-action-force-n", record: "ChangedColumnActionForceN" };

    fn diff(&self, base: &En1994Snapshot) -> protocol::MutationOutcome<<En1994Mutation as protocol::Mutation<En1994Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1994Snapshot) -> Vec<En1994Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("change-column-action-force-n", "change-column-action-force-n")
    }
}
//#endregion 🔖️Payload
