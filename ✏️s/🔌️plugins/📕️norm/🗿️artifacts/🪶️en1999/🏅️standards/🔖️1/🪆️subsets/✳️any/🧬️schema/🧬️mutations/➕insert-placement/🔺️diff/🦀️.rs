//! 🔺️ `insert-placement` diff.

use crate::insert_placement::InsertPlacement;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999PlacementsRows};

pub fn diff(payload: &InsertPlacement, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if base.placements.iter().any(|existing| existing.id == payload.placement.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Placement id {} already exists.", payload.placement.id), [payload.placement.id.clone()]);
    }
    if (payload.index as usize) > base.placements.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", (payload.index as usize), base.placements.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1999Diff { placements: Some(En1999PlacementsRows::insertion(payload.index as usize, payload.placement.clone())), ..Default::default() })
}
