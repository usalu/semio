//! 🚁 `insert-helicopter` diff: inserts the helicopter landing row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_helicopter::InsertHelicopter;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991HelicopterLandingDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &InsertHelicopter, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.helicopter_landings.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Helicopter landing {key} already exists."), [key]);
    }
    if payload.index > base.helicopter_landings.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.helicopter_landings.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1991Diff { helicopter_landings: En1991HelicopterLandingDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
