//! 🔺️ Diff for `change-slab-action-xm`.
use crate::change_slab_action_x_m::ChangeSlabActionXM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
use semio_s_artifact_norm_en1994_diff::{En1994Diff, En1994SlabsRows, En1994SlabsPatch, En1994SlabsActionsRows, En1994SlabsActionsPatch};

pub fn diff(payload: &ChangeSlabActionXM, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    let Some(index) = base.slabs.iter().position(|row| row.id == payload.slab_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("slab {} does not exist.", payload.slab_id), [payload.slab_id.clone()]);
    };
    let Some(action_index) = base.slabs[index].actions.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("action {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if !payload.new_x_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "must be finite", [index.to_string()]);
    }
    let Some(slab) = base.slabs.get(index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "slab missing", [index.to_string()]);
    };
    let Some(action) = slab.actions.get(action_index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "action missing", [action_index.to_string()]);
    };
    if (action.x_m - payload.new_x_m).abs() < f64::EPSILON {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff {
        slabs: Some(En1994SlabsRows::modification(&payload.slab_id, En1994SlabsPatch {
                actions: Some(En1994SlabsActionsRows::modification(&payload.id, En1994SlabsActionsPatch { x_m: Some(payload.new_x_m), ..Default::default() })),
                ..Default::default()
            })),
        ..Default::default()
    })
}
