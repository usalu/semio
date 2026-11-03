//! 📍️ `insert-placement` diff: inserts the placement row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_placement::InsertPlacement;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997PlacementsRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &InsertPlacement, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if base.placements.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Placement {key} already exists."), [key]);
    }
    if payload.index > base.placements.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.placements.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1997Diff { placements: Some(En1997PlacementsRows::insertion(payload.index, payload.item.clone())), ..Default::default() })
}
