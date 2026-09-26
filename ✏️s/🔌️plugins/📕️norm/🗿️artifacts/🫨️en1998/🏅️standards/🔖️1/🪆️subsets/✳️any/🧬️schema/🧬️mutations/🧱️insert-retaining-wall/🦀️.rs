//! 🧱️ `insert-retaining-wall` mutation leaf.

use crate::{En1998Mutation, En1998Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertRetainingWall {
    pub index: usize,
    pub wall: crate::En1998RetainingWall,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for InsertRetainingWall {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "insert",
        entity: "retaining-wall",
        kind: "insert-retaining-wall",
        record: "InsertRetainingWall",
    };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<<En1998Mutation as protocol::Mutation<En1998Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("insert-retaining-wall", "insert-retaining-wall")
    }
    fn target(&self) -> Vec<String> {
        vec!["insert-retaining-wall".into()]
    }
}
