//! ↩️ `remove-selection-constraint` — undo re-`add`s the captured constraint (at the end, not
//! necessarily its original position — `add` has no index arg); out-of-range BASE index ⇒
//! `Vec::new()`.

use crate::artifacts::iso16757::mutations::add_selection_constraint;
use crate::artifacts::iso16757::{Iso16757Mutation, Iso16757Snapshot};

use super::mutation::RemoveSelectionConstraint;

//#region 🔖️Inverse
pub fn inverse(payload: &RemoveSelectionConstraint, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
    match base.selection.constraints.get(payload.index) {
        Some(constraint) => vec![Iso16757Mutation::AddSelectionConstraint(add_selection_constraint::mutation::AddSelectionConstraint { constraint: constraint.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
