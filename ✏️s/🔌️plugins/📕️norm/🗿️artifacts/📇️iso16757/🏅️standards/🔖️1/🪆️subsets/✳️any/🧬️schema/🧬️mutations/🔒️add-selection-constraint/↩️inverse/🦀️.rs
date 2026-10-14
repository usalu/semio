//! ↩️ `add-selection-constraint` — undo is `remove-selection-constraint` at the position the constraint landed on.

use crate::remove_selection_constraint;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

use crate::add_selection_constraint::mutation::AddSelectionConstraint;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &AddSelectionConstraint, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_selection_constraint::mutation::RemoveSelectionConstraint> {
    Ok((|| {
    if base.selection.constraints.contains(&payload.constraint) {
        return Vec::new();
    }
    let len = base.selection.constraints.len();
    let at = payload.index.filter(|index| *index <= len).unwrap_or(len);
    vec![Op::from(remove_selection_constraint::mutation::RemoveSelectionConstraint { index: at })]

    })())
}
//#endregion 🔖️Inverse
