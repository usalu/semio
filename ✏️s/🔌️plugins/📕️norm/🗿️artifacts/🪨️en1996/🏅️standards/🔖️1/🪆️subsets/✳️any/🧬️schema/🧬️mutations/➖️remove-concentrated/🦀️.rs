//! ➖️remove-concentrated
use crate::{En1996Mutation, En1996Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveConcentrated {
    pub wall_index: usize,
    pub load_case_index: usize,
    pub index: usize,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for RemoveConcentrated {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "remove",
        entity: "concentrated",
        kind: "remove-concentrated",
        record: "RemovedConcentrated",
    };
    fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<<En1996Mutation as protocol::Mutation<En1996Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Remove concentrated action", "Einzellast entfernen")
    }
}

#[cfg(test)]
#[path = "🧪️tests/➖️applies-remove-concentrated/🦀️.rs"]
mod named_test;
