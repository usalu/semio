//! 🏭️change-masonry-class
use crate::{En1996Mutation, En1996Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeMasonryClass {
    pub new_masonry_class: crate::MasonryClass,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for ChangeMasonryClass {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "masonry-class",
        kind: "change-masonry-class",
        record: "ChangedMasonryClass",
    };
    fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<<En1996Mutation as protocol::Mutation<En1996Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Change masonry execution class", "Mauerwerksklasse ändern")
    }
}

#[cfg(test)]
#[path = "🧪️tests/🏭️applies-change-masonry-class/🦀️.rs"]
mod named_test;
