//! ↩️ `change-selection-class` — undo restores BASE's class id.

use crate::change_selection_class::mutation::ChangeSelectionClass;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &ChangeSelectionClass, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_selection_class::mutation::ChangeSelectionClass> {
    Ok((|| {
    vec![Op::from(ChangeSelectionClass { new_class_id: base.selection.class_id.clone() })]

    })())
}
//#endregion 🔖️Inverse
