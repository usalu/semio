//! ↩️ `insert-layer` — undo is `remove-layer` at the (clamped) FINAL-state index the layer landed
//! at, which is also a valid BASE-state index for the follow-up removal.

use super::mutation::InsertLayer;
use crate::artifacts::din4108::mutations::remove_layer;
use crate::artifacts::din4108::{Din4108Mutation, Din4108Snapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &InsertLayer, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    let at = payload.index.min(base.layers.len());
    vec![Din4108Mutation::RemoveLayer(remove_layer::mutation::RemoveLayer { index: at })]
}
//#endregion 🔖️Inverse
