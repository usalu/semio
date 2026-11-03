//! 📍️ `insert-placement` diff: inserts the placement row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_placement::InsertPlacement;
use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992PlacementsRows};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &InsertPlacement, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if base.placements.iter().any(|existing| existing.id == payload.placement.id) {
        let key = payload.placement.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Placement {key} already exists."), [key]);
    }
    if payload.index > base.placements.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.placements.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1992Diff { placements: Some(En1992PlacementsRows::insertion(payload.index, payload.placement.clone())), ..Default::default() })
}
