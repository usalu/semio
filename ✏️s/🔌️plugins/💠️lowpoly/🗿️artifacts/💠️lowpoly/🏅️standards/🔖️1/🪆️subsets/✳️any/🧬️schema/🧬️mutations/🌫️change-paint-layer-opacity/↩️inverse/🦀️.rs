//! ↩️ `change-paint-layer-opacity` — undo restores the base-state opacity; missing object/index ⇒ `Vec::new()`.

use super::ChangePaintLayerOpacity;
use crate::artifacts::lowpoly::{LowpolyMutation, LowpolySnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &ChangePaintLayerOpacity, base: &LowpolySnapshot) -> Vec<LowpolyMutation> {
    let Some(layer) = base.objects.iter().find(|object| object.id == payload.object_id).and_then(|object| object.paint_layers.get(payload.index)) else {
        return Vec::new();
    };
    vec![LowpolyMutation::ChangePaintLayerOpacity(ChangePaintLayerOpacity { object_id: payload.object_id.clone(), index: payload.index, new_opacity: layer.opacity })]
}
//#endregion 🔖️Inverse
