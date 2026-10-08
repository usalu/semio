//! Diff for `change-slab-action-q-area-pa`.
use super::ChangeSlabActionQAreaPa;
use crate::{En1994Snapshot};
use crate::diff::{En1994Diff, En1994SlabsRows, En1994SlabsPatch, En1994SlabsActionsRows, En1994SlabsActionsPatch};

pub fn diff(payload: &ChangeSlabActionQAreaPa, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if !payload.new_q_area_pa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "must be finite", [payload.index.to_string()]);
    }
    let Some(slab) = base.slabs.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "slab missing", [payload.index.to_string()]);
    };
    let Some(action) = slab.actions.get(payload.action_index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "action missing", [payload.action_index.to_string()]);
    };
    if (action.q_area_pa - payload.new_q_area_pa).abs() < f64::EPSILON {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff {
        slabs: Some(En1994SlabsRows {
            modified: vec![En1994SlabsPatch {
                index: payload.index,
                actions: Some(En1994SlabsActionsRows { modified: vec![En1994SlabsActionsPatch { index: payload.action_index, q_area_pa: Some(payload.new_q_area_pa), ..Default::default() }] }),
                ..Default::default()
            }],
            ..Default::default()
        }),
        ..Default::default()
    })
}
