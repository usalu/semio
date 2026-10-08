//! Diff for `change-beam-stud-fu-pa`.
use super::ChangeBeamStudFUPa;
use crate::{En1994Snapshot};
use crate::diff::{En1994Diff, En1994BeamsRows, En1994BeamsPatch};

pub fn diff(payload: &ChangeBeamStudFUPa, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if !payload.new_f_u_pa.is_finite() || payload.new_f_u_pa <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "must be positive finite", [payload.index.to_string()]);
    }
    let Some(beam) = base.beams.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "beam missing", [payload.index.to_string()]);
    };
    if beam.studs.f_u_pa == payload.new_f_u_pa {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff {
        beams: Some(En1994BeamsRows { modified: vec![En1994BeamsPatch { index: payload.index, studs_f_u_pa: Some(payload.new_f_u_pa), ..Default::default() }], ..Default::default() }),
        ..Default::default()
    })
}
