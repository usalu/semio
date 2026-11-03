//! 📍️ `insert-placement` diff: inserts the placement row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_placement::InsertPlacement;
use semio_s_artifact_norm_en1996_diff::{En1996Diff, En1996PlacementDelta};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &InsertPlacement, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if base.placements.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("placement {key} already exists."), [key]);
    }
    if payload.index > base.placements.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.placements.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1996Diff { placements: En1996PlacementDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
