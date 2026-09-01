//! 📏 `change-self-weight-thickness-m` — sets the En1991 self-weight thickness scalar.


use crate::artifacts::en1991::{En1991Diff, En1991Mutation, En1991Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeSelfWeightThicknessM {
    pub new_self_weight_thickness_m: f64,
}

impl protocol::MutationKind<En1991Snapshot, En1991Mutation> for ChangeSelfWeightThicknessM {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "self-weight-thickness-m", kind: "change-self-weight-thickness-m", record: "ChangedSelfWeightThicknessM" };

    fn diff(&self, base: &En1991Snapshot) -> protocol::MutationOutcome<<En1991Mutation as protocol::Mutation<En1991Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1991Snapshot) -> Vec<En1991Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change self-weight thickness to {:?}", self.new_self_weight_thickness_m)
    }
}
//#endregion 🔖️Payload
