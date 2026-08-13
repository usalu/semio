//! 🔺️ `change-variable-action-value` — sparse diff construction; an out-of-range BASE index is a
//! no-op clone (nothing at that position to change). Reads `base` through the `en1990_qk`
//! working-scene accessor and re-mints a fresh content-addressed child handle (ticket
//! 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM round 2).

use super::mutation::ChangeVariableActionValue;
use crate::artifacts::en1990::{en1990_qk, en1990_qk_child_from_entries, En1990Diff, En1990Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeVariableActionValue, base: &En1990Snapshot) -> En1990Diff {
    let mut q_k = en1990_qk(base);
    if let Some(entry) = q_k.get_mut(payload.index) {
        entry.value = payload.new_value;
    }
    En1990Diff { q_k: Some(en1990_qk_child_from_entries(&q_k)), ..Default::default() }
}
//#endregion 🔖️Diff
