//! 🔩change-as-vertical
use crate::{En1996Mutation, En1996Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeAsVertical {
    pub index: usize,
    pub new_as_vertical_m2: f64,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for ChangeAsVertical {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "as-vertical",
        kind: "change-as-vertical",
        record: "ChangedAsVertical",
    };
    fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<<En1996Mutation as protocol::Mutation<En1996Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Change vertical reinforcement As", "Vertikale Bewehrung As ändern")
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔩applies-change-as-vertical/🦀️.rs"]
mod named_test;
