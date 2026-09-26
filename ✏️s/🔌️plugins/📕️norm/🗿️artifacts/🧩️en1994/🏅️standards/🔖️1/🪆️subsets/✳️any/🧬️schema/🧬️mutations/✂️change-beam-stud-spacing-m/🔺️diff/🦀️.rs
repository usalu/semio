//! Diff for `change-beam-stud-spacing-m`.
use super::ChangeBeamStudSpacingM;
use crate::diff::En1994BeamList;
use crate::{En1994Diff, En1994Snapshot};
pub fn diff(payload: &ChangeBeamStudSpacingM, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if !payload.new_spacing_m.is_finite() || payload.new_spacing_m <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "spacing must be positive", [payload.index.to_string()]);
    }
    let Some(beam) = base.beams.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "beam missing", [payload.index.to_string()]);
    };
    if (beam.studs.spacing_m - payload.new_spacing_m).abs() < f64::EPSILON {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "unchanged");
    }
    let mut beams = base.beams.clone();
    beams[payload.index].studs.spacing_m = payload.new_spacing_m;
    protocol::MutationOutcome::new(En1994Diff { beams: Some(En1994BeamList { values: beams }), ..Default::default() })
}
