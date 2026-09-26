//! Diff for `change-beam-slab-thickness-m`.
use super::ChangeBeamSlabThicknessM;
use crate::diff::En1994BeamList;
use crate::{En1994Diff, En1994Snapshot};
pub fn diff(payload: &ChangeBeamSlabThicknessM, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if !payload.new_slab_thickness_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "must be finite", [payload.index.to_string()]);
    }
    let Some(beam) = base.beams.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "beam missing", [payload.index.to_string()]);
    };
    if beam.slab_thickness_m == payload.new_slab_thickness_m {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "unchanged");
    }
    let mut beams = base.beams.clone();
    beams[payload.index].slab_thickness_m = payload.new_slab_thickness_m;
    protocol::MutationOutcome::new(En1994Diff { beams: Some(En1994BeamList { values: beams }), ..Default::default() })
}
