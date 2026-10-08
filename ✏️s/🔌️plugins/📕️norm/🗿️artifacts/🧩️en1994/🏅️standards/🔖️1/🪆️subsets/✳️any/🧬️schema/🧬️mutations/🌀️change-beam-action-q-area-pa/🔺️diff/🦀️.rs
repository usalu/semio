//! Diff for `change-beam-action-q-area-pa`.
use super::ChangeBeamActionQAreaPa;
use crate::{En1994Snapshot};
use crate::diff::{En1994Diff, En1994BeamsRows, En1994BeamsPatch, En1994BeamsActionsRows, En1994BeamsActionsPatch};

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
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff {
        beams: Some(En1994BeamsRows {
            modified: vec![En1994BeamsPatch {
                index: payload.index,
                actions: Some(En1994BeamsActionsRows { modified: vec![En1994BeamsActionsPatch { index: payload.action_index, q_area_pa: Some(payload.new_q_area_pa), ..Default::default() }] }),
                ..Default::default()
            }],
            ..Default::default()
        }),
        ..Default::default()
    })
}
