//! 📏️ `change-storey-drift-x-m` mutation leaf.

use crate::{En1998Mutation, En1998Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeStoreyDriftXM {
    pub building_index: usize,
    pub storey_index: usize,
    pub new_drift_x_m: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeStoreyDriftXM {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "storey-drift-x-m",
        kind: "change-storey-drift-xm",
        record: "ChangeStoreyDriftXM",
    };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<<En1998Mutation as protocol::Mutation<En1998Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("change-storey-drift-x-m", "change-storey-drift-x-m")
    }
    fn target(&self) -> Vec<String> {
        vec!["change-storey-drift-x-m".into()]
    }
}
