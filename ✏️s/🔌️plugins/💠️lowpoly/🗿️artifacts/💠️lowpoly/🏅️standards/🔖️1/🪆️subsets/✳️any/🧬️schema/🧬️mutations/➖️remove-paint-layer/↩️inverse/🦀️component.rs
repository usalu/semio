//! ↩️ `remove-paint-layer` — undo re-`insert-paint-layer`s the captured layer at the same index;
//! missing object/index ⇒ `Vec::new()`.

use super::mutation::RemovePaintLayer;
use crate::artifacts::lowpoly::mutations::insert_paint_layer;
use crate::artifacts::lowpoly::{LowpolyMutation, LowpolySnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &RemovePaintLayer, base: &LowpolySnapshot) -> Vec<LowpolyMutation> {
    let Some(layer) = base.objects.iter().find(|object| object.id == payload.object_id).and_then(|object| object.paint_layers.get(payload.index)) else {
        return Vec::new();
    };
    vec![LowpolyMutation::InsertPaintLayer(insert_paint_layer::mutation::InsertPaintLayer { object_id: payload.object_id.clone(), index: payload.index, layer: layer.clone() })]
}
//#endregion 🔖️Inverse
