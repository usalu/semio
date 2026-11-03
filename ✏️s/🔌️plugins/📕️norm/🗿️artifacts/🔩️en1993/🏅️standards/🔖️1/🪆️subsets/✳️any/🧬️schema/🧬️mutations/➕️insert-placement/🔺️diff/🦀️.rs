//! ➕️ `insert-placement` diff — inserts the row at its position, clamped to the end of the collection.

use crate::insert_placement::InsertPlacement;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993PlacementDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &InsertPlacement, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.placements.iter().any(|existing| existing.id == payload.placement.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Placement id {} already exists.", payload.placement.id), [payload.placement.id.clone()]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.placements.len());
    protocol::MutationOutcome::new(En1993Diff { placements: En1993PlacementDelta::insertion(index, payload.placement.clone()), ..Default::default() })
}
