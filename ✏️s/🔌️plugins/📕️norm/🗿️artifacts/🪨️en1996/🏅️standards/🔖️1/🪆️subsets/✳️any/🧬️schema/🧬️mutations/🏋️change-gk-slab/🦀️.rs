//! 🏋️change-gk-slab
use crate::{En1996Mutation, En1996Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeGKSlab {
    pub wall_index: usize,
    pub index: usize,
    pub new_g_k_slab_n: f64,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for ChangeGKSlab {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change", entity: "g-k-slab", kind: "change-gk-slab", record: "ChangedGKSlab",
    };
    fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<<En1996Mutation as protocol::Mutation<En1996Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Change slab permanent action Gk", "Ständige Deckenlast Gk ändern")
    }
}

#[cfg(test)]
#[path = "🧪️tests/🏋️applies-change-gk-slab/🦀️.rs"]
mod named_test;
