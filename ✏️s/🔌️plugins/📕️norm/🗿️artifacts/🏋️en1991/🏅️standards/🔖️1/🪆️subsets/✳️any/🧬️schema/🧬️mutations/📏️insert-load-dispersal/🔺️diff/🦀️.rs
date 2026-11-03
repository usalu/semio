//! 📏 `insert-load-dispersal` diff: inserts the load dispersal row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_load_dispersal::InsertLoadDispersal;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991LoadDispersalDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &InsertLoadDispersal, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.load_dispersals.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Load dispersal {key} already exists."), [key]);
    }
    if payload.index > base.load_dispersals.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.load_dispersals.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1991Diff { load_dispersals: En1991LoadDispersalDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
