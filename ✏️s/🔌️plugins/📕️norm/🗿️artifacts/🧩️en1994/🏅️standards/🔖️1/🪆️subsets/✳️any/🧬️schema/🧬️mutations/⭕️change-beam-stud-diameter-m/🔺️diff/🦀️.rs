//! Diff for `change-beam-stud-diameter-m`.
use super::ChangeBeamStudDiameterM;
use crate::{En1994Snapshot};
use crate::diff::{En1994Diff, En1994BeamsRows, En1994BeamsPatch};

pub fn diff(payload: &ChangeBeamStudDiameterM, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if !payload.new_diameter_m.is_finite() || payload.new_diameter_m <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "must be positive finite", [payload.index.to_string()]);
    }
    let Some(beam) = base.beams.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "beam missing", [payload.index.to_string()]);
    };
    if beam.studs.diameter_m == payload.new_diameter_m {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff {
        beams: Some(En1994BeamsRows { modified: vec![En1994BeamsPatch { index: payload.index, studs_diameter_m: Some(payload.new_diameter_m), ..Default::default() }], ..Default::default() }),
        ..Default::default()
    })
}
