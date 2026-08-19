//! 🔺️ `change-selection-series` — sparse diff construction.

use super::mutation::ChangeSelectionSeries;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ChangeSelectionSeries, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.selection.series_id == payload.new_series_id {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Selection series already has this value.");
    }
    let mut selection = base.selection.clone();
    selection.series_id = payload.new_series_id.clone();
    protocol::MutationOutcome::new(Iso16757Diff { selection: Some(selection), ..Default::default() })
}
//#endregion 🔖️Diff
