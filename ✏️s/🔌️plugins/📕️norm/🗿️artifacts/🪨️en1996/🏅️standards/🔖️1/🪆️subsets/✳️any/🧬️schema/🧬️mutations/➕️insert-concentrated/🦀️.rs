//! ➕️insert-concentrated
use crate::{En1996Mutation, En1996Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertConcentrated {
    pub wall_index: usize,
    pub load_case_index: usize,
    pub index: usize,
    pub load: crate::ConcentratedLoad,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for InsertConcentrated {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "insert",
        entity: "concentrated",
        kind: "insert-concentrated",
        record: "InsertedConcentrated",
    };
    fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<<En1996Mutation as protocol::Mutation<En1996Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Insert concentrated action", "Einzellast einfügen")
    }
}

#[cfg(test)]
#[path = "🧪️tests/➕️applies-insert-concentrated/🦀️.rs"]
mod named_test;
