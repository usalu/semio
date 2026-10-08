//! ➕️ `insert-fatigue-detail` diff — inserts the row at its position, clamped to the end of the collection.

use super::InsertFatigueDetail;
use crate::diff::En1993RowEdit as _;
use crate::diff::{En1993Diff, En1993FatigueDetailEdit};
use crate::En1993Snapshot;

pub fn diff(payload: &InsertFatigueDetail, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.fatigue_details.iter().any(|existing| existing.id == payload.fatigue_detail.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Fatigue detail id {} already exists.", payload.fatigue_detail.id), [payload.fatigue_detail.id.clone()]);
    }
    let index = payload.index.min(base.fatigue_details.len());
    protocol::MutationOutcome::new(En1993Diff { fatigue_details: vec![En1993FatigueDetailEdit::insert(index, payload.fatigue_detail.clone())], ..Default::default() })
}
