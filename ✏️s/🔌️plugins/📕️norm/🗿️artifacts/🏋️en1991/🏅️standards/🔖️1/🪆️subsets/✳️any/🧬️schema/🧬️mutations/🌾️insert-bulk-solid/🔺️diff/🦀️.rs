//! 🌾 `insert-bulk-solid` diff: inserts the bulk solid row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_bulk_solid::InsertBulkSolid;
use semio_s_artifact_norm_en1991_diff::{En1991BulkSolidDelta, En1991Diff};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &InsertBulkSolid, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.bulk_solids.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Bulk solid {key} already exists."), [key]);
    }
    if payload.index > base.bulk_solids.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.bulk_solids.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1991Diff { bulk_solids: En1991BulkSolidDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
