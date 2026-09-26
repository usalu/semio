//! 📐️change-concentrated-bearing-area
use crate::{En1996Mutation, En1996Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeConcentratedBearingArea {
    pub wall_index: usize,
    pub load_case_index: usize,
    pub index: usize,
    pub new_bearing_area_m2: f64,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for ChangeConcentratedBearingArea {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "concentrated-bearing-area",
        kind: "change-concentrated-bearing-area",
        record: "ChangedConcentratedBearingArea",
    };
    fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<<En1996Mutation as protocol::Mutation<En1996Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Change concentrated bearing area", "Lastfläche der Einzellast ändern")
    }
}

#[cfg(test)]
#[path = "🧪️tests/📐️applies-change-concentrated-bearing-area/🦀️.rs"]
mod named_test;
