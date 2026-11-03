//! 📍️ `insert-placement` diff on the positional `placements` lane.

use crate::insert_placement::InsertPlacement;
use semio_s_artifact_norm_en1994_diff::{En1994Diff, En1994PlacementsInserted, En1994PlacementsRows};
use semio_s_artifact_norm_en1994_model::En1994Snapshot;

pub fn diff(payload: &InsertPlacement, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if base.placements.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Placement {key} already exists."), [key]);
    }
    if payload.index > base.placements.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.placements.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1994Diff { placements: Some(En1994PlacementsRows::insertion(payload.index, payload.item.clone())), ..Default::default() })
}
