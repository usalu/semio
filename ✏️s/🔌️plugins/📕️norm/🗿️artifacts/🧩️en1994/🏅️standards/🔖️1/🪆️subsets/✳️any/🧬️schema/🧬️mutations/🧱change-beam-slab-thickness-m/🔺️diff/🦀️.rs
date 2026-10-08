//! Diff for `change-beam-slab-thickness-m`.
use super::ChangeBeamSlabThicknessM;
use crate::{En1994Snapshot};
use crate::diff::{En1994Diff, En1994BeamsRows, En1994BeamsPatch};

pub fn diff(payload: &ChangeBeamSlabThicknessM, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if !payload.new_slab_thickness_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "must be finite", [payload.index.to_string()]);
    }
    let Some(beam) = base.beams.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "beam missing", [payload.index.to_string()]);
    };
    if beam.slab_thickness_m == payload.new_slab_thickness_m {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff {
        beams: Some(En1994BeamsRows { modified: vec![En1994BeamsPatch { index: payload.index, slab_thickness_m: Some(payload.new_slab_thickness_m), ..Default::default() }], ..Default::default() }),
        ..Default::default()
    })
}
