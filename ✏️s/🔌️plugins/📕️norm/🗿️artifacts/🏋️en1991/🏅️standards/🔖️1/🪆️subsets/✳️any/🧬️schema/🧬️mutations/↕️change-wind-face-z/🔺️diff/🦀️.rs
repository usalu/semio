//! ↕️ `change-wind-face-z` diff: patches `z` of the addressed wind face row.

use crate::change_wind_face_z::ChangeWindFaceZ;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991WindFaceDelta, En1991WindFacePatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeWindFaceZ, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_z.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.wind_faces.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("wind face {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.z == payload.new_z {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The wind face already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { wind_faces: En1991WindFaceDelta::modification(&payload.id, En1991WindFacePatch { z: Some(payload.new_z), ..Default::default() }), ..Default::default() })
}
