//! ➕️ `insert-floor` diff: inserts the floor row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_floor::InsertFloor;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991FloorDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &InsertFloor, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.floors.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("floor {key} already exists."), [key]);
    }
    if payload.index > base.floors.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.floors.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1991Diff { floors: En1991FloorDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
