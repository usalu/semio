//! 🔺️ `insert-variable-action` — sparse diff construction. `En1990Diff.q_k` is a single-`Option`
//! composed-child slot (ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM round 2) — every `q_k`
//! mutation rebuilds the full ordered entry list from `base` (via the `en1990_qk` working-scene
//! accessor, not the old direct field read) and re-mints a fresh content-addressed child handle
//! via `en1990_qk_child_from_entries`, exactly `➗️mathematical`'s equivalent per-mutation pattern.

use super::mutation::InsertVariableAction;
use crate::artifacts::en1990::{en1990_qk, en1990_qk_child_from_entries, En1990Diff, En1990QkEntry, En1990Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &InsertVariableAction, base: &En1990Snapshot) -> En1990Diff {
    let mut q_k = en1990_qk(base);
    let at = payload.index.min(q_k.len());
    q_k.insert(at, En1990QkEntry { category: payload.category.clone(), value: payload.value });
    En1990Diff { q_k: Some(en1990_qk_child_from_entries(&q_k)), ..Default::default() }
}
//#endregion 🔖️Diff
