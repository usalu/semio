//! 💪️ `change-beam-stud-fu-pa` mutation leaf.

use crate::{En1994Mutation, En1994Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeBeamStudFUPa {
    pub index: usize,
    pub new_f_u_pa: f64,
}

impl protocol::MutationKind<En1994Snapshot, En1994Mutation> for ChangeBeamStudFUPa {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "beam-stud-fu-pa", kind: "change-beam-stud-fu-pa", record: "ChangedBeamStudFUPa" };

    fn diff(&self, base: &En1994Snapshot) -> protocol::MutationOutcome<<En1994Mutation as protocol::Mutation<En1994Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1994Snapshot) -> Vec<En1994Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("change-beam-stud-fu-pa", "change-beam-stud-fu-pa")
    }
}
//#endregion 🔖️Payload
