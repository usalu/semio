//! 📏️ `change-layer-thickness` — sets one construction layer's `thickness_m`, addressed by
//! BASE-state index.


use crate::artifacts::din4108::{Din4108Diff, Din4108Mutation, Din4108Snapshot};
use crate::artifacts::din4108::diff::Din4108LayerList;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeLayerThickness {
    pub index: usize,
    pub new_thickness_m: f64,
}

impl protocol::MutationKind<Din4108Snapshot, Din4108Mutation> for ChangeLayerThickness {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "layer-thickness", kind: "change-layer-thickness", record: "ChangedLayerThickness" };

    fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change layer #{} thickness to {}", self.index, self.new_thickness_m)
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}
//#endregion 🔖️Payload
