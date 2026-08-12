//! 🔺️ `insert-variable-action` — sparse diff construction. `En1990Diff.q_k` is a whole-list-per-
//! diff wrapper (`En1990QkList`), not a sparse triple — every `q_k` mutation rebuilds the full
//! ordered `values` vec from `base` and wraps it.

use super::mutation::InsertVariableAction;
use crate::artifacts::en1990::diff::En1990QkList;
use crate::artifacts::en1990::{En1990Diff, En1990QkEntry, En1990Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &InsertVariableAction, base: &En1990Snapshot) -> En1990Diff {
    let mut q_k = base.q_k.clone();
    let at = payload.index.min(q_k.len());
    q_k.insert(at, En1990QkEntry { category: payload.category.clone(), value: payload.value });
    En1990Diff { q_k: Some(En1990QkList { values: q_k }), ..Default::default() }
}
//#endregion 🔖️Diff
