//! 🏠️ `change-roof-surface-pitch-deg` diff: patches `pitch_deg` of the addressed roof surface row.

use crate::change_roof_surface_pitch_deg::ChangeRoofSurfacePitchDeg;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991RoofSurfaceDelta, En1991RoofSurfacePatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeRoofSurfacePitchDeg, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_pitch_deg.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.roof_surfaces.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("roof surface {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.pitch_deg == payload.new_pitch_deg {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The roof surface already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { roof_surfaces: En1991RoofSurfaceDelta::modification(&payload.id, En1991RoofSurfacePatch { pitch_deg: Some(payload.new_pitch_deg), ..Default::default() }), ..Default::default() })
}
