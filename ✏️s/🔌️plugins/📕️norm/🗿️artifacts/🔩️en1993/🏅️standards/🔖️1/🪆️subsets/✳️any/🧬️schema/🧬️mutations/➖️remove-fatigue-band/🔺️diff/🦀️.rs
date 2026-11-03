//! ➖️ `remove-fatigue-band` diff: removes the element at the index from the addressed row's list.

use crate::remove_fatigue_band::RemoveFatigueBand;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FatigueDetailDelta, En1993FatigueDetailPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &RemoveFatigueBand, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.fatigue_details.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "fatigue-band", Vec::<String>::new());
    };
    if payload.index >= row.spectrum.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("fatigue-band index {} out of range.", payload.index), Vec::<String>::new());
    }
    let mut value = row.spectrum.clone();
    value.remove(payload.index);
    protocol::MutationOutcome::new(En1993Diff { fatigue_details: En1993FatigueDetailDelta::modification(payload.id.clone(), En1993FatigueDetailPatch { spectrum: Some(value), ..Default::default() }), ..Default::default() })
}
