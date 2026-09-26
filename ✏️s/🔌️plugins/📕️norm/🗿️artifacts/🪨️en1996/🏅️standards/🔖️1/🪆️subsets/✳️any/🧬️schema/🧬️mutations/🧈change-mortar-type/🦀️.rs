//! 🧈change-mortar-type
use crate::{En1996Mutation, En1996Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeMortarType {
    pub index: usize,
    pub new_mortar_type: crate::MortarType,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for ChangeMortarType {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "mortar-type",
        kind: "change-mortar-type",
        record: "ChangedMortarType",
    };
    fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<<En1996Mutation as protocol::Mutation<En1996Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Change mortar type", "Mörtelart ändern")
    }
}

#[cfg(test)]
#[path = "🧪️tests/🧈applies-change-mortar-type/🦀️.rs"]
mod named_test;
