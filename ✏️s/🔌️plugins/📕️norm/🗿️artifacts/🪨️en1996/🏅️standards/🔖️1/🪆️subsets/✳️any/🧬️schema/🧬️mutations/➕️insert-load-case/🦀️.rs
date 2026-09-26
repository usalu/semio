//! ➕️insert-load-case
use crate::{En1996Mutation, En1996Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertLoadCase {
    pub wall_index: usize,
    pub index: usize,
    pub load_case: crate::WallLoadCase,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for InsertLoadCase {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "insert",
        entity: "load-case",
        kind: "insert-load-case",
        record: "InsertedLoadCase",
    };
    fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<<En1996Mutation as protocol::Mutation<En1996Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Insert load case", "Lastfall einfügen")
    }
}

#[cfg(test)]
#[path = "🧪️tests/➕️applies-insert-load-case/🦀️.rs"]
mod named_test;
