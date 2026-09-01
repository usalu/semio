//! ↩️ `change-paint-layer-visible` — undo restores the base-state flag; missing object/index ⇒ `Vec::new()`.

use super::ChangePaintLayerVisible;
use crate::artifacts::lowpoly::{LowpolyMutation, LowpolySnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &ChangePaintLayerVisible, base: &LowpolySnapshot) -> Vec<LowpolyMutation> {
    let Some(layer) = base.objects.iter().find(|object| object.id == payload.object_id).and_then(|object| object.paint_layers.get(payload.index)) else {
        return Vec::new();
    };
    vec![LowpolyMutation::ChangePaintLayerVisible(ChangePaintLayerVisible { object_id: payload.object_id.clone(), index: payload.index, new_visible: layer.visible })]
}
//#endregion 🔖️Inverse
