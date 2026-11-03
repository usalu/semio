//! 🧱️ `insert-slab-action` diff on the keyed `actions` lane of `slabs`.

use crate::insert_slab_action::InsertSlabAction;
use semio_s_artifact_norm_en1994_diff::{En1994SlabsActionsInserted, En1994SlabsActionsRows, En1994SlabsPatch, En1994SlabsRows, En1994Diff};
use semio_s_artifact_norm_en1994_model::En1994Snapshot;

pub fn diff(payload: &InsertSlabAction, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    let Some(parent) = base.slabs.iter().find(|row| row.id == payload.slab_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Slab {} does not exist.", payload.slab_id), [payload.slab_id.clone()]);
    };
    if parent.actions.iter().any(|existing| existing.id == payload.action.id) {
        let key = payload.action.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Slab action {key} already exists."), [key]);
    }
    if payload.index > parent.actions.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, parent.actions.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1994Diff { slabs: Some(En1994SlabsRows::modification(&payload.slab_id, En1994SlabsPatch { actions: Some(En1994SlabsActionsRows::insertion(payload.index, payload.action.clone())), ..Default::default() })), ..Default::default() })
}
