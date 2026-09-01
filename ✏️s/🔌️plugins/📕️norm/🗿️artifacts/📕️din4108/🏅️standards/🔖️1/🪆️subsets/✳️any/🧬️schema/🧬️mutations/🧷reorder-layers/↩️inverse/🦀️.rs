//! ↩️ `reorder-layers` — undo moves the layer back: `reorder{from: min(to, len-1), to: from}`
//! (`📓️taxonomy.md`'s addressing convention #3); out-of-range BASE `from` ⇒ `Vec::new()`.

use super::ReorderLayers;
use crate::artifacts::din4108::{Din4108Mutation, Din4108Snapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &ReorderLayers, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    let len = base.layers.len();
    if len == 0 || payload.from >= len {
        return Vec::new();
    }
    let landed_at = payload.to.min(len - 1);
    vec![Din4108Mutation::ReorderLayers(ReorderLayers { from: landed_at, to: payload.from })]
}
//#endregion 🔖️Inverse
