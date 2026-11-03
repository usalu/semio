//! ⚙ `insert-support-collision` diff: inserts the support collision row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_support_collision::InsertSupportCollision;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991SupportCollisionDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &InsertSupportCollision, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.support_collisions.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Support collision {key} already exists."), [key]);
    }
    if payload.index > base.support_collisions.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.support_collisions.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1991Diff { support_collisions: En1991SupportCollisionDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
