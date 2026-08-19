//! ↩️ `add-selection-constraint` — undo is `remove-selection-constraint` at the index the append
//! landed on (BASE length, since the new constraint always lands at the end).

use crate::artifacts::iso16757::mutations::remove_selection_constraint;
use crate::artifacts::iso16757::{Iso16757Mutation, Iso16757Snapshot};

use super::mutation::AddSelectionConstraint;

//#region 🔖️Inverse
pub async fn inverse(_payload: &AddSelectionConstraint, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
    vec![Iso16757Mutation::RemoveSelectionConstraint(remove_selection_constraint::mutation::RemoveSelectionConstraint { index: base.selection.constraints.len() })]
}
//#endregion 🔖️Inverse
