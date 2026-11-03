//! 📐 `change-roof-pitch-deg` diff: patches `pitch_deg` of the addressed roof row.

use crate::change_roof_pitch_deg::ChangeRoofPitchDeg;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991RoofDelta, En1991RoofPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeRoofPitchDeg, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_pitch_deg.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.roofs.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("roof {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.pitch_deg == payload.new_pitch_deg {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The roof already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { roofs: En1991RoofDelta::modification(&payload.id, En1991RoofPatch { pitch_deg: Some(payload.new_pitch_deg), ..Default::default() }), ..Default::default() })
}
