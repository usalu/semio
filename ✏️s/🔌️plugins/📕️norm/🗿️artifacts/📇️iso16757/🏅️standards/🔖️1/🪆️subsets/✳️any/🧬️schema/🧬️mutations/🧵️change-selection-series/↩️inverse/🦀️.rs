//! ↩️ `change-selection-series` — undo restores BASE's series id.

use crate::change_selection_series::mutation::ChangeSelectionSeries;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &ChangeSelectionSeries, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_selection_series::mutation::ChangeSelectionSeries> {
    Ok((|| {
    vec![Op::from(ChangeSelectionSeries { new_series_id: base.selection.series_id.clone() })]

    })())
}
//#endregion 🔖️Inverse
