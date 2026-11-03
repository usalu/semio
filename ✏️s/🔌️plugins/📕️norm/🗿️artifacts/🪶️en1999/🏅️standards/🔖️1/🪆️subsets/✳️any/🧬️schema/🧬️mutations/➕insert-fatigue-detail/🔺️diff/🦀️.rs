//! 🔺️ `insert-fatigue-detail` diff.

use crate::insert_fatigue_detail::InsertFatigueDetail;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999FatigueDetailsRows};

pub fn diff(payload: &InsertFatigueDetail, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if base.fatigue_details.iter().any(|existing| existing.id == payload.fatigue_detail.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Fatigue detail id {} already exists.", payload.fatigue_detail.id), [payload.fatigue_detail.id.clone()]);
    }
    if (payload.index as usize) > base.fatigue_details.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", (payload.index as usize), base.fatigue_details.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1999Diff { fatigue_details: Some(En1999FatigueDetailsRows::insertion(payload.index as usize, payload.fatigue_detail.clone())), ..Default::default() })
}
