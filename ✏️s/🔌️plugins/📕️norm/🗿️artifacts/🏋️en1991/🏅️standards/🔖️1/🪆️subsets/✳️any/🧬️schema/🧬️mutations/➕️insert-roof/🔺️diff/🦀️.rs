//! ➕️ `insert-roof` diff: inserts the roof row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_roof::InsertRoof;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991RoofDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &InsertRoof, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.roofs.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("roof {key} already exists."), [key]);
    }
    if payload.index > base.roofs.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.roofs.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1991Diff { roofs: En1991RoofDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
