//! ↩️ `change-paint-layer-blend-mode` — undo restores the base-state blend mode; missing
//! object/index ⇒ `Vec::new()`.

use super::mutation::ChangePaintLayerBlendMode;
use crate::artifacts::lowpoly::{LowpolyMutation, LowpolySnapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &ChangePaintLayerBlendMode, base: &LowpolySnapshot) -> Vec<LowpolyMutation> {
    let Some(layer) = base.objects.iter().find(|object| object.id == payload.object_id).and_then(|object| object.paint_layers.get(payload.index)) else {
        return Vec::new();
    };
    vec![LowpolyMutation::ChangePaintLayerBlendMode(ChangePaintLayerBlendMode { object_id: payload.object_id.clone(), index: payload.index, new_blend_mode: layer.blend_mode.clone() })]
}
//#endregion 🔖️Inverse
