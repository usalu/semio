//! ⬇ `insert-falling` diff: inserts the falling load row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_falling::InsertFalling;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991FallingLoadDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &InsertFalling, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.falling_loads.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Falling load {key} already exists."), [key]);
    }
    if payload.index > base.falling_loads.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.falling_loads.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1991Diff { falling_loads: En1991FallingLoadDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
