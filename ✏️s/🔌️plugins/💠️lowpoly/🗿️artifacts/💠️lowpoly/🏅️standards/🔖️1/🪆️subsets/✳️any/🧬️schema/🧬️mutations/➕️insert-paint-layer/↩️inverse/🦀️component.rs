//! ↩️ `insert-paint-layer` — undo is `remove-paint-layer` at the same (now-final) index.

use super::mutation::InsertPaintLayer;
use crate::artifacts::lowpoly::mutations::remove_paint_layer;
use crate::artifacts::lowpoly::{LowpolyMutation, LowpolySnapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &InsertPaintLayer, _base: &LowpolySnapshot) -> Vec<LowpolyMutation> {
    vec![LowpolyMutation::RemovePaintLayer(remove_paint_layer::mutation::RemovePaintLayer { object_id: payload.object_id.clone(), index: payload.index })]
}
//#endregion 🔖️Inverse
