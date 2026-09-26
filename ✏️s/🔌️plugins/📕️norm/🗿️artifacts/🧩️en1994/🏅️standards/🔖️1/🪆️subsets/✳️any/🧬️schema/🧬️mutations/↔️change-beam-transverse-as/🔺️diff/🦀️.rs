//! Diff for `change-beam-transverse-as`.
use super::ChangeBeamTransverseAs;
use crate::diff::En1994BeamList;
use crate::{En1994Diff, En1994Snapshot};
pub fn diff(payload: &ChangeBeamTransverseAs, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if !payload.new_transverse_as_m2_per_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "must be finite", [payload.index.to_string()]);
    }
    let Some(beam) = base.beams.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "beam missing", [payload.index.to_string()]);
    };
    if beam.transverse_as_m2_per_m == payload.new_transverse_as_m2_per_m {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "unchanged");
    }
    let mut beams = base.beams.clone();
    beams[payload.index].transverse_as_m2_per_m = payload.new_transverse_as_m2_per_m;
    protocol::MutationOutcome::new(En1994Diff { beams: Some(En1994BeamList { values: beams }), ..Default::default() })
}
