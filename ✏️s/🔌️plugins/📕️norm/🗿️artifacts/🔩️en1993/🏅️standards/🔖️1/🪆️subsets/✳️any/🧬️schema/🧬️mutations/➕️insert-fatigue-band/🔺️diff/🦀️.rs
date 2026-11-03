//! ➕️ `insert-fatigue-band` diff: inserts the element into the addressed row's list, clamped to its end.

use crate::insert_fatigue_band::InsertFatigueBand;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FatigueDetailDelta, En1993FatigueDetailPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &InsertFatigueBand, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.fatigue_details.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "fatigue-band", Vec::<String>::new());
    };
    if row.spectrum.iter().any(|existing| existing.id == payload.element.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Fatigue band id {} already exists.", payload.element.id), [payload.element.id.clone()]);
    }
    let mut value = row.spectrum.clone();
    let at = payload.index.unwrap_or(usize::MAX).min(value.len());
    value.insert(at, payload.element.clone());
    protocol::MutationOutcome::new(En1993Diff { fatigue_details: En1993FatigueDetailDelta::modification(payload.id.clone(), En1993FatigueDetailPatch { spectrum: Some(value), ..Default::default() }), ..Default::default() })
}
