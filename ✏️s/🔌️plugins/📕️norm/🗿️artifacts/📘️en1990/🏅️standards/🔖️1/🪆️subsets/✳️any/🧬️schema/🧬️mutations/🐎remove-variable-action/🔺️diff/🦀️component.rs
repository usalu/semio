//! 🔺️ `remove-variable-action` — sparse diff construction; an out-of-range BASE index is a no-op
//! clone (nothing to remove). Reads `base` through the `en1990_qk` working-scene accessor and
//! re-mints a fresh content-addressed child handle (ticket
//! 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM round 2), same pattern as every sibling triad.

use super::mutation::RemoveVariableAction;
use crate::artifacts::en1990::{en1990_qk, en1990_qk_child_from_entries, En1990Diff, En1990Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &RemoveVariableAction, base: &En1990Snapshot) -> En1990Diff {
    let mut q_k = en1990_qk(base);
    if payload.index < q_k.len() {
        q_k.remove(payload.index);
    }
    En1990Diff { q_k: Some(en1990_qk_child_from_entries(&q_k)), ..Default::default() }
}
//#endregion 🔖️Diff
