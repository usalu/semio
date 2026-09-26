//! ↔️change-concentrated-bearing-length
use crate::{En1996Mutation, En1996Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeConcentratedBearingLength {
    pub wall_index: usize,
    pub load_case_index: usize,
    pub index: usize,
    pub new_bearing_length_m: f64,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for ChangeConcentratedBearingLength {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "concentrated-bearing-length",
        kind: "change-concentrated-bearing-length",
        record: "ChangedConcentratedBearingLength",
    };
    fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<<En1996Mutation as protocol::Mutation<En1996Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Change concentrated bearing length", "Auflagerlänge der Einzellast ändern")
    }
}

#[cfg(test)]
#[path = "🧪️tests/↔️applies-change-concentrated-bearing-length/🦀️.rs"]
mod named_test;
