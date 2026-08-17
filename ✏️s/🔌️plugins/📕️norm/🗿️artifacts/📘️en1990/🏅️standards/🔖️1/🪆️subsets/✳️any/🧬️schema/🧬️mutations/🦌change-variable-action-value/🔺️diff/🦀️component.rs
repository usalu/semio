//! 🔺️ `change-variable-action-value` — sparse diff construction; an out-of-range BASE index is a
//! no-op clone (nothing at that position to change). Reads `base` through the `en1990_qk`
//! working-scene accessor and re-mints a fresh content-addressed child handle (ticket
//! 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM round 2).

use super::mutation::ChangeVariableActionValue;
use crate::artifacts::en1990::{en1990_qk, en1990_qk_child_from_entries, En1990Diff, En1990Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeVariableActionValue, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    if !payload.new_value.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Variable action value must be a finite number.", [payload.index.to_string()]);
    }
    let mut q_k = en1990_qk(base);
    let Some(entry) = q_k.get_mut(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Variable action #{} does not exist.", payload.index), [payload.index.to_string()]);
    };
    if entry.value == payload.new_value {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Variable action #{} already has this value.", payload.index));
    }
    entry.value = payload.new_value;
    protocol::MutationOutcome::new(En1990Diff { q_k: Some(en1990_qk_child_from_entries(&q_k)), ..Default::default() })
}
//#endregion 🔖️Diff
