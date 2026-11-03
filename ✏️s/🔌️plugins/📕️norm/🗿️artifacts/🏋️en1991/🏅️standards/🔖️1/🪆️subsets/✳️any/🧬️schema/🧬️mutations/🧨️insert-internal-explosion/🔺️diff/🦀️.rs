//! 🧨 `insert-internal-explosion` diff: inserts the internal explosion row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_internal_explosion::InsertInternalExplosion;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991InternalExplosionDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &InsertInternalExplosion, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.internal_explosions.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Internal explosion {key} already exists."), [key]);
    }
    if payload.index > base.internal_explosions.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.internal_explosions.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1991Diff { internal_explosions: En1991InternalExplosionDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
