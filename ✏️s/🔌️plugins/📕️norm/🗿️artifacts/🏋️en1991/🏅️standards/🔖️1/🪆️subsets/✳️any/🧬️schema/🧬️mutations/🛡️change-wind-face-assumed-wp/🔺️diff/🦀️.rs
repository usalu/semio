//! 🛡️ `change-wind-face-assumed-wp` diff: patches `assumed_wp` of the addressed wind face row.

use crate::change_wind_face_assumed_wp::ChangeWindFaceAssumedWp;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991WindFaceDelta, En1991WindFacePatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeWindFaceAssumedWp, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_assumed_wp.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.wind_faces.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("wind face {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.assumed_wp == payload.new_assumed_wp {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The wind face already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { wind_faces: En1991WindFaceDelta::modification(&payload.id, En1991WindFacePatch { assumed_wp: Some(payload.new_assumed_wp), ..Default::default() }), ..Default::default() })
}
