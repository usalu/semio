//! 🔺️ `change-variable-action-value` — sparse diff construction; an out-of-range BASE index is a
//! no-op clone (nothing at that position to change).

use super::mutation::ChangeVariableActionValue;
use crate::artifacts::en1990::diff::En1990QkList;
use crate::artifacts::en1990::{En1990Diff, En1990Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeVariableActionValue, base: &En1990Snapshot) -> En1990Diff {
    let mut q_k = base.q_k.clone();
    if let Some(entry) = q_k.get_mut(payload.index) {
        entry.value = payload.new_value;
    }
    En1990Diff { q_k: Some(En1990QkList { values: q_k }), ..Default::default() }
}
//#endregion 🔖️Diff
