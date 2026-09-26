//! 🧮change-c-pe
use crate::{En1996Mutation, En1996Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeCPe {
    pub wall_index: usize,
    pub index: usize,
    pub new_c_pe: f64,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for ChangeCPe {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change", entity: "c-pe", kind: "change-c-pe", record: "ChangedCPe",
    };
    fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<<En1996Mutation as protocol::Mutation<En1996Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Change external pressure coefficient cpe", "Außendruckbeiwert cpe ändern")
    }
}

#[cfg(test)]
#[path = "🧪️tests/🧮applies-change-c-pe/🦀️.rs"]
mod named_test;
