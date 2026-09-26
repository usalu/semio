//! Inverse for `change-column-kind`.
use super::ChangeColumnKind;
use crate::{En1994Mutation, En1994Snapshot};
pub fn inverse(payload: &ChangeColumnKind, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    let Some(col) = base.columns.get(payload.index) else { return Vec::new(); };
    vec![En1994Mutation::ChangeColumnKind(ChangeColumnKind { index: payload.index, new_kind: col.kind.clone() })]
}
