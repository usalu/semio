//! 📍️ `change-seismic-slope-friction-angle` diff: patches `friction-angle` of the addressed seismic_slope row.

use crate::change_seismic_slope_friction_angle::ChangeSeismicSlopeFrictionAngle;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998SlopeDelta, En1998SlopePatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeSeismicSlopeFrictionAngle, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_friction_angle.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.slopes.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("SeismicSlope {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.friction_angle == payload.new_friction_angle {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The seismic_slope already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { slopes: En1998SlopeDelta::modification(&payload.id, En1998SlopePatch { friction_angle: Some(payload.new_friction_angle), ..Default::default() }), ..Default::default() })
}
