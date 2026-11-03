//! ⚙ `insert-local-fire` diff: inserts the localised fire row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_local_fire::InsertLocalFire;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991LocalFireDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &InsertLocalFire, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.local_fires.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Localised fire {key} already exists."), [key]);
    }
    if payload.index > base.local_fires.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.local_fires.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1991Diff { local_fires: En1991LocalFireDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
