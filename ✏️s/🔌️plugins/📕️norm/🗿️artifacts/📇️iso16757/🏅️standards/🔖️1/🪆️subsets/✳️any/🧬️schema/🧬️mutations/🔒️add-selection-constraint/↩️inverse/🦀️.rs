//! ↩️ `add-selection-constraint` — undo is `remove-selection-constraint` at the position the constraint landed on.

use crate::mutations::remove_selection_constraint;
use crate::{Iso16757Mutation, Iso16757Snapshot};

use super::mutation::AddSelectionConstraint;

//#region 🔖️Inverse
pub fn inverse(payload: &AddSelectionConstraint, base: &Iso16757Snapshot) -> Result<Vec<Iso16757Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if base.selection.constraints.contains(&payload.constraint) {
        return Vec::new();
    }
    let len = base.selection.constraints.len();
    let at = payload.index.filter(|index| *index <= len).unwrap_or(len);
    vec![Iso16757Mutation::RemoveSelectionConstraint(remove_selection_constraint::mutation::RemoveSelectionConstraint { index: at })]

    })())
}
//#endregion 🔖️Inverse
