//! 🪚️ `insert-chase` diff: inserts the chase row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_chase::InsertChase;
use semio_s_artifact_norm_en1996_diff::{En1996ChaseDelta, En1996Diff};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &InsertChase, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if base.chases.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Chase {key} already exists."), [key]);
    }
    if payload.index > base.chases.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.chases.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1996Diff { chases: En1996ChaseDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
