//! 🔺️ `change-selection-series` — sparse diff construction.

use crate::change_selection_series::mutation::ChangeSelectionSeries;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;
use semio_s_artifact_norm_iso16757_diff::{Iso16757Diff, Iso16757SelectionSeriesIdValue};

//#region 🔖️Diff

pub fn diff(payload: &ChangeSelectionSeries, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.selection.series_id == payload.new_series_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Selection series already has this value.");
    }
    protocol::MutationOutcome::new(Iso16757Diff { selection_series_id: Some(Iso16757SelectionSeriesIdValue { value: payload.new_series_id.clone() }), ..Default::default() })
}
