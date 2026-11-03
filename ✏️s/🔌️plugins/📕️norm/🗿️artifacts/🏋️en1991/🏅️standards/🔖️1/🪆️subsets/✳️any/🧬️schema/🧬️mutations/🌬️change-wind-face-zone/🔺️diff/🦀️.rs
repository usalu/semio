//! 🌬 `change-wind-face-zone` diff: patches `zone` of the addressed wind face row.

use crate::change_wind_face_zone::ChangeWindFaceZone;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991WindFaceDelta, En1991WindFacePatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeWindFaceZone, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(r0) = base.wind_faces.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Wind face {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.zone == payload.new_zone {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The wind face already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { wind_faces: En1991WindFaceDelta::modification(&payload.id, En1991WindFacePatch { zone: Some(payload.new_zone.clone()), ..Default::default() }), ..Default::default() })
}
