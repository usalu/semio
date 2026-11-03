//! ⚙ `insert-snow-guard` diff: inserts the snow guard row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_snow_guard::InsertSnowGuard;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991SnowGuardDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &InsertSnowGuard, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.snow_guards.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Snow guard {key} already exists."), [key]);
    }
    if payload.index > base.snow_guards.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.snow_guards.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1991Diff { snow_guards: En1991SnowGuardDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
