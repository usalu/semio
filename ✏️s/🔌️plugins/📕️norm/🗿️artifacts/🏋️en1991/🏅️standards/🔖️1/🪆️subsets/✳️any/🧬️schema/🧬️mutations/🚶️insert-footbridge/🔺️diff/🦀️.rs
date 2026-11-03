//! 🚶 `insert-footbridge` diff: inserts the footbridge row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_footbridge::InsertFootbridge;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991FootbridgeDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &InsertFootbridge, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.footbridges.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Footbridge {key} already exists."), [key]);
    }
    if payload.index > base.footbridges.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.footbridges.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1991Diff { footbridges: En1991FootbridgeDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
