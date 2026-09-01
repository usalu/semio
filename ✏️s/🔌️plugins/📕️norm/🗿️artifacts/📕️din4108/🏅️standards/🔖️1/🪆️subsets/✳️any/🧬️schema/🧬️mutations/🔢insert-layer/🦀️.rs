//! ➕️ `insert-layer` — places a new construction layer at a FINAL-state index in the layer
//! build-up (an intrinsically ordered, anonymous collection — no stable id on `LayerDocument`).


use crate::artifacts::din4108::{Din4108Diff, Din4108Mutation, Din4108Snapshot, LayerDocument};
use crate::artifacts::din4108::diff::Din4108LayerList;
use crate::artifacts::din4108::mutations::remove_layer;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertLayer {
    pub index: usize,
    pub layer: LayerDocument,
}

impl protocol::MutationKind<Din4108Snapshot, Din4108Mutation> for InsertLayer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "layer", kind: "insert-layer", record: "InsertedLayer" };

    fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Insert layer at #{}", self.index)
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}
//#endregion 🔖️Payload
