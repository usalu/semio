//! Inverse for `change-column-action-force-n`.
use super::ChangeColumnActionForceN;
use crate::{En1994Mutation, En1994Snapshot};
pub fn inverse(payload: &ChangeColumnActionForceN, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    let Some(col) = base.columns.get(payload.index) else { return Vec::new(); };
    let Some(action) = col.actions.get(payload.action_index) else { return Vec::new(); };
    vec![En1994Mutation::ChangeColumnActionForceN(ChangeColumnActionForceN { index: payload.index, action_index: payload.action_index, new_n_k_n: action.n_k_n })]
}
