//! 🔺️ `reorder-variable-actions` — sparse diff construction; an out-of-range BASE `from` is a
//! no-op clone. Reads `base` through the `en1990_qk` working-scene accessor and re-mints a fresh
//! content-addressed child handle (ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM round 2).

use super::ReorderVariableActions;
use crate::artifacts::en1990::{en1990_qk, en1990_qk_child_from_entries, En1990Diff, En1990Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ReorderVariableActions, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    let mut q_k = en1990_qk(base);
    if payload.from >= q_k.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Variable action #{} does not exist.", payload.from), [payload.from.to_string()]);
    }
    let at = payload.to.min(q_k.len() - 1);
    if at == payload.from {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Variable action #{} is already at position #{}.", payload.from, at));
    }
    let item = q_k.remove(payload.from);
    q_k.insert(at, item);
    protocol::MutationOutcome::new(En1990Diff { q_k: Some(en1990_qk_child_from_entries(&q_k)), ..Default::default() })
}
//#endregion 🔖️Diff
