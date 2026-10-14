//! ↩️ `remove-selection-constraint` — undo re-`add`s the captured constraint at its original position; out-of-range BASE index ⇒
//! `Vec::new()`.

use crate::add_selection_constraint;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

use crate::remove_selection_constraint::mutation::RemoveSelectionConstraint;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &RemoveSelectionConstraint, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::add_selection_constraint::mutation::AddSelectionConstraint> {
    Ok((|| {
    match base.selection.constraints.get(payload.index) {
        Some(constraint) => vec![Op::from(add_selection_constraint::mutation::AddSelectionConstraint { constraint: constraint.clone(), index: Some(payload.index) })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
