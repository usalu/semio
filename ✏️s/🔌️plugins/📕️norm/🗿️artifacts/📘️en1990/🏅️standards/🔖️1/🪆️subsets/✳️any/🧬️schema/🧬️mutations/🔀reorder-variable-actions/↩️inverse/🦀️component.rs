//! ↩️ `reorder-variable-actions` — undo moves the entry back: `reorder{from: min(to, len-1), to:
//! from}` (`📓️taxonomy.md`'s addressing convention #3); out-of-range BASE `from` ⇒ `Vec::new()`.

use super::mutation::ReorderVariableActions;
use crate::artifacts::en1990::{En1990Mutation, En1990Snapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &ReorderVariableActions, base: &En1990Snapshot) -> Vec<En1990Mutation> {
    let len = base.q_k.len();
    if len == 0 || payload.from >= len {
        return Vec::new();
    }
    let landed_at = payload.to.min(len - 1);
    vec![En1990Mutation::ReorderVariableActions(ReorderVariableActions { from: landed_at, to: payload.from })]
}
//#endregion 🔖️Inverse
