//! ↩️ `remove-layer` — undo re-`insert`s the captured layer at its original BASE-state index;
//! out-of-range BASE index ⇒ `Vec::new()`.

use super::mutation::RemoveLayer;
use crate::artifacts::din4108::mutations::insert_layer;
use crate::artifacts::din4108::{Din4108Mutation, Din4108Snapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &RemoveLayer, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    match base.layers.get(payload.index) {
        Some(layer) => vec![Din4108Mutation::InsertLayer(insert_layer::mutation::InsertLayer { index: payload.index, layer: layer.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
