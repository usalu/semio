//! ➕️ `insert-paint-layer` — places a new paint layer into an object's ordered (compositing-order)
//! layer list at a FINAL-state index; layers have no stable id, only position.

use crate::artifacts::lowpoly::{LowpolyMutation, LowpolyPaintLayer, LowpolySnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct InsertPaintLayer {
    pub object_id: String,
    pub index: usize,
    pub layer: LowpolyPaintLayer,
}

impl protocol::MutationKind<LowpolySnapshot, LowpolyMutation> for InsertPaintLayer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "paint-layer", kind: "insert-paint-layer", record: "InsertedPaintLayer" };

    fn diff(&self, base: &LowpolySnapshot) -> protocol::MutationOutcome<<LowpolyMutation as protocol::Mutation<LowpolySnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &LowpolySnapshot) -> Vec<LowpolyMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Insert paint layer \"{}\"", self.layer.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.object_id.clone()]
    }
}
//#endregion 🔖️Payload
