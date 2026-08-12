//! 🔺️ `reorder-variable-actions` — sparse diff construction; an out-of-range BASE `from` is a
//! no-op clone.

use super::mutation::ReorderVariableActions;
use crate::artifacts::en1990::diff::En1990QkList;
use crate::artifacts::en1990::{En1990Diff, En1990Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ReorderVariableActions, base: &En1990Snapshot) -> En1990Diff {
    let mut q_k = base.q_k.clone();
    if payload.from < q_k.len() {
        let item = q_k.remove(payload.from);
        let at = payload.to.min(q_k.len());
        q_k.insert(at, item);
    }
    En1990Diff { q_k: Some(En1990QkList { values: q_k }), ..Default::default() }
}
//#endregion 🔖️Diff
