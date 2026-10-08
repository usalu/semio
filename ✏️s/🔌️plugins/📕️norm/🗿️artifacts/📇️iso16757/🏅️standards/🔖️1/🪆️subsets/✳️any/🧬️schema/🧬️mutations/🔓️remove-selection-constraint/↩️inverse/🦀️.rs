//! ↩️ `remove-selection-constraint` — undo re-`add`s the captured constraint at its original position; out-of-range BASE index ⇒
//! `Vec::new()`.

use crate::mutations::add_selection_constraint;
use crate::{Iso16757Mutation, Iso16757Snapshot};

use super::mutation::RemoveSelectionConstraint;

//#region 🔖️Inverse
pub fn inverse(payload: &RemoveSelectionConstraint, base: &Iso16757Snapshot) -> Result<Vec<Iso16757Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.selection.constraints.get(payload.index) {
        Some(constraint) => vec![Iso16757Mutation::AddSelectionConstraint(add_selection_constraint::mutation::AddSelectionConstraint { constraint: constraint.clone(), index: Some(payload.index) })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
