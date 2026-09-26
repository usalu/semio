//! `change-beam-action-q-area-pa` mutation leaf.

use crate::{En1994Mutation, En1994Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeBeamActionQAreaPa {
    pub index: usize,
    pub action_index: usize,
    pub new_q_area_pa: f64,
}

impl protocol::MutationKind<En1994Snapshot, En1994Mutation> for ChangeBeamActionQAreaPa {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "beam-action-q-area-pa", kind: "change-beam-action-q-area-pa", record: "ChangedBeamActionQAreaPa" };

    fn diff(&self, base: &En1994Snapshot) -> protocol::MutationOutcome<<En1994Mutation as protocol::Mutation<En1994Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1994Snapshot) -> Vec<En1994Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("change-beam-action-q-area-pa", "change-beam-action-q-area-pa")
    }
}
//#endregion 🔖️Payload
