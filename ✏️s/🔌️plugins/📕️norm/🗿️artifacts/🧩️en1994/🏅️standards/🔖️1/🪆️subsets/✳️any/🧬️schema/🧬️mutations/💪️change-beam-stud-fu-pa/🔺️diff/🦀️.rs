//! Diff for `change-beam-stud-fu-pa`.
use super::ChangeBeamStudFUPa;
use crate::diff::En1994BeamList;
use crate::{En1994Diff, En1994Snapshot};
pub fn diff(payload: &ChangeBeamStudFUPa, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if !payload.new_f_u_pa.is_finite() || payload.new_f_u_pa <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "must be positive finite", [payload.index.to_string()]);
    }
    let Some(beam) = base.beams.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "beam missing", [payload.index.to_string()]);
    };
    if beam.studs.f_u_pa == payload.new_f_u_pa {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "unchanged");
    }
    let mut beams = base.beams.clone();
    beams[payload.index].studs.f_u_pa = payload.new_f_u_pa;
    protocol::MutationOutcome::new(En1994Diff { beams: Some(En1994BeamList { values: beams }), ..Default::default() })
}
