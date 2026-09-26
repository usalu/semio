//! ➕️insert-wall
use crate::{En1996Mutation, En1996Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertWall {
    pub index: usize,
    pub wall: crate::MasonryWall,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for InsertWall {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "insert",
        entity: "wall",
        kind: "insert-wall",
        record: "InsertedWall",
    };
    fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<<En1996Mutation as protocol::Mutation<En1996Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Insert wall", "Wand einfügen")
    }
}

#[cfg(test)]
#[path = "🧪️tests/➕️applies-insert-wall/🦀️.rs"]
mod named_test;
