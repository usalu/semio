//! 🧱change-density
use crate::{En1996Mutation, En1996Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeDensity {
    pub index: usize,
    pub new_density_kg_m3: f64,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for ChangeDensity {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "density",
        kind: "change-density",
        record: "ChangedDensity",
    };
    fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<<En1996Mutation as protocol::Mutation<En1996Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Change masonry density", "Rohdichte des Mauerwerks ändern")
    }
}

#[cfg(test)]
#[path = "🧪️tests/🧱applies-change-density/🦀️.rs"]
mod named_test;
