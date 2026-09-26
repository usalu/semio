//! ↘️change-eccentricity-top
use crate::{En1996Mutation, En1996Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeEccentricityTop {
    pub index: usize,
    pub new_eccentricity_top_m: f64,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for ChangeEccentricityTop {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "eccentricity-top",
        kind: "change-eccentricity-top",
        record: "ChangedEccentricityTop",
    };
    fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<<En1996Mutation as protocol::Mutation<En1996Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Change top eccentricity", "Exzentrizität oben ändern")
    }
}

#[cfg(test)]
#[path = "🧪️tests/↘️applies-change-eccentricity-top/🦀️.rs"]
mod named_test;
