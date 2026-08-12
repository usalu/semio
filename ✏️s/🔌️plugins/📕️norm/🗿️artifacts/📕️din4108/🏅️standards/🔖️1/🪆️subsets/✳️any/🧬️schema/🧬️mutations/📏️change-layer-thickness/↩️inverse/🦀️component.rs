//! ↩️ `change-layer-thickness` — undo restores BASE's thickness at that index; out-of-range BASE
//! index ⇒ `Vec::new()`.

use super::mutation::ChangeLayerThickness;
use crate::artifacts::din4108::{Din4108Mutation, Din4108Snapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &ChangeLayerThickness, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    match base.layers.get(payload.index) {
        Some(layer) => vec![Din4108Mutation::ChangeLayerThickness(ChangeLayerThickness { index: payload.index, new_thickness_m: layer.thickness_m })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
