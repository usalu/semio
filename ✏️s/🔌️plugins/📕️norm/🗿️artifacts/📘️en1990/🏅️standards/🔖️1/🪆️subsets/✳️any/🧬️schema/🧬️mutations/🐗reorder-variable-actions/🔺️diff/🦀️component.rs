//! 🔺️ `reorder-variable-actions` — sparse diff construction; an out-of-range BASE `from` is a
//! no-op clone. Reads `base` through the `en1990_qk` working-scene accessor and re-mints a fresh
//! content-addressed child handle (ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM round 2).

use super::mutation::ReorderVariableActions;
use crate::artifacts::en1990::{en1990_qk, en1990_qk_child_from_entries, En1990Diff, En1990Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ReorderVariableActions, base: &En1990Snapshot) -> En1990Diff {
    let mut q_k = en1990_qk(base);
    if payload.from < q_k.len() {
        let item = q_k.remove(payload.from);
        let at = payload.to.min(q_k.len());
        q_k.insert(at, item);
    }
    En1990Diff { q_k: Some(en1990_qk_child_from_entries(&q_k)), ..Default::default() }
}
//#endregion 🔖️Diff
