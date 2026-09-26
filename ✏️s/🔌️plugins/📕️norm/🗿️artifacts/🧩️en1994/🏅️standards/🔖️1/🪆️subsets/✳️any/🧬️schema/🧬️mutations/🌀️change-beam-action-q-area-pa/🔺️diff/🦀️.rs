//! Diff for `change-beam-action-q-area-pa`.
use super::ChangeBeamActionQAreaPa;
use crate::diff::En1994BeamList;
use crate::{En1994Diff, En1994Snapshot};
pub fn diff(payload: &ChangeBeamActionQAreaPa, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if !payload.new_q_area_pa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "must be finite", [payload.index.to_string()]);
    }
    let Some(beam) = base.beams.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "beam missing", [payload.index.to_string()]);
    };
    let Some(action) = beam.actions.get(payload.action_index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "action missing", [payload.action_index.to_string()]);
    };
    if (action.q_area_pa - payload.new_q_area_pa).abs() < f64::EPSILON {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "unchanged");
    }
    let mut beams = base.beams.clone();
    beams[payload.index].actions[payload.action_index].q_area_pa = payload.new_q_area_pa;
    protocol::MutationOutcome::new(En1994Diff { beams: Some(En1994BeamList { values: beams }), ..Default::default() })
}
