//! ↩️ `rename-paint-layer` — undo restores the base-state name; missing object/index ⇒ `Vec::new()`.

use super::mutation::RenamePaintLayer;
use crate::artifacts::lowpoly::{LowpolyMutation, LowpolySnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &RenamePaintLayer, base: &LowpolySnapshot) -> Vec<LowpolyMutation> {
    let Some(layer) = base.objects.iter().find(|object| object.id == payload.object_id).and_then(|object| object.paint_layers.get(payload.index)) else {
        return Vec::new();
    };
    vec![LowpolyMutation::RenamePaintLayer(RenamePaintLayer { object_id: payload.object_id.clone(), index: payload.index, new_name: layer.name.clone() })]
}
//#endregion 🔖️Inverse
