//! 🔺️ `remove-variable-action` — sparse diff construction; an out-of-range BASE index is a no-op
//! clone (nothing to remove).

use super::mutation::RemoveVariableAction;
use crate::artifacts::en1990::diff::En1990QkList;
use crate::artifacts::en1990::{En1990Diff, En1990Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &RemoveVariableAction, base: &En1990Snapshot) -> En1990Diff {
    let mut q_k = base.q_k.clone();
    if payload.index < q_k.len() {
        q_k.remove(payload.index);
    }
    En1990Diff { q_k: Some(En1990QkList { values: q_k }), ..Default::default() }
}
//#endregion 🔖️Diff
