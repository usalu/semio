//! 🔺️ `change-selection-series` — sparse diff construction.

use super::mutation::ChangeSelectionSeries;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeSelectionSeries, base: &Iso16757Snapshot) -> Iso16757Diff {
    let mut selection = base.selection.clone();
    selection.series_id = payload.new_series_id.clone();
    Iso16757Diff { selection: Some(selection), ..Default::default() }
}
//#endregion 🔖️Diff
