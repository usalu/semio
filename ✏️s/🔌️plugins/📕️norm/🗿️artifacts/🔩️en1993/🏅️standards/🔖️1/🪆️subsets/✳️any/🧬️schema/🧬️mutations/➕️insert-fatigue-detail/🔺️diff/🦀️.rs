//! ➕️ `insert-fatigue-detail` diff — inserts the row at its position, clamped to the end of the collection.

use crate::insert_fatigue_detail::InsertFatigueDetail;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FatigueDetailDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &InsertFatigueDetail, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.fatigue_details.iter().any(|existing| existing.id == payload.fatigue_detail.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Fatigue detail id {} already exists.", payload.fatigue_detail.id), [payload.fatigue_detail.id.clone()]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.fatigue_details.len());
    protocol::MutationOutcome::new(En1993Diff { fatigue_details: En1993FatigueDetailDelta::insertion(index, payload.fatigue_detail.clone()), ..Default::default() })
}
