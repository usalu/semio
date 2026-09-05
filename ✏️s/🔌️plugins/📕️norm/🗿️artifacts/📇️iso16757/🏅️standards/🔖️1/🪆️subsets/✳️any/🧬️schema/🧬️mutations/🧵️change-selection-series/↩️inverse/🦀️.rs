//! ↩️ `change-selection-series` — undo restores BASE's series id.

use super::mutation::ChangeSelectionSeries;
use crate::artifacts::iso16757::{Iso16757Mutation, Iso16757Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeSelectionSeries, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
    vec![Iso16757Mutation::ChangeSelectionSeries(ChangeSelectionSeries { new_series_id: base.selection.series_id.clone() })]
}
//#endregion 🔖️Inverse
