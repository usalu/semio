//! 🧱change-support-sides
use crate::{En1996Mutation, En1996Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeSupportSides {
    pub index: usize,
    pub new_support_sides: u8,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for ChangeSupportSides {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "support-sides",
        kind: "change-support-sides",
        record: "ChangedSupportSides",
    };
    fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<<En1996Mutation as protocol::Mutation<En1996Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Change number of supported sides", "Anzahl der gehaltenen Ränder ändern")
    }
}

#[cfg(test)]
#[path = "🧪️tests/🧱applies-change-support-sides/🦀️.rs"]
mod named_test;
