//! 🧱️ `remove-slab-action` diff on the keyed `actions` lane of `slabs`.

use crate::remove_slab_action::RemoveSlabAction;
use semio_s_artifact_norm_en1994_diff::{En1994SlabsActionsRemoved, En1994SlabsActionsRows, En1994SlabsPatch, En1994SlabsRows, En1994Diff};
use semio_s_artifact_norm_en1994_model::En1994Snapshot;

pub fn diff(payload: &RemoveSlabAction, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    let Some(parent) = base.slabs.iter().find(|row| row.id == payload.slab_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Slab {} does not exist.", payload.slab_id), [payload.slab_id.clone()]);
    };
    let Some(index) = parent.actions.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Slab action {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1994Diff { slabs: Some(En1994SlabsRows::modification(&payload.slab_id, En1994SlabsPatch { actions: Some(En1994SlabsActionsRows::removal(&parent.actions, index)), ..Default::default() })), ..Default::default() })
}
