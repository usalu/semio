//! Diff for `change-slab-action-q-area-pa`.
use super::ChangeSlabActionQAreaPa;
use crate::diff::En1994SlabList;
use crate::{En1994Diff, En1994Snapshot};
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
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "unchanged");
    }
    let mut slabs = base.slabs.clone();
    slabs[payload.index].actions[payload.action_index].q_area_pa = payload.new_q_area_pa;
    protocol::MutationOutcome::new(En1994Diff { slabs: Some(En1994SlabList { values: slabs }), ..Default::default() })
}
