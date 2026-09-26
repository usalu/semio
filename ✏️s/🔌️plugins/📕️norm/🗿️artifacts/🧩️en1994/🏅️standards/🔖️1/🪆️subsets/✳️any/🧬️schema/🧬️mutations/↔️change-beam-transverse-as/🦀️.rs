//! ↔️ `change-beam-transverse-as` mutation leaf.

use crate::{En1994Mutation, En1994Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeBeamTransverseAs {
    pub index: usize,
    pub new_transverse_as_m2_per_m: f64,
}

impl protocol::MutationKind<En1994Snapshot, En1994Mutation> for ChangeBeamTransverseAs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "beam-transverse-as", kind: "change-beam-transverse-as", record: "ChangedBeamTransverseAs" };

    fn diff(&self, base: &En1994Snapshot) -> protocol::MutationOutcome<<En1994Mutation as protocol::Mutation<En1994Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1994Snapshot) -> Vec<En1994Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("change-beam-transverse-as", "change-beam-transverse-as")
    }
}
//#endregion 🔖️Payload
