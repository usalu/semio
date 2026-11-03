//! ➖️ `remove-wind-face` diff: removes the addressed wind face row.

use crate::remove_wind_face::RemoveWindFace;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991WindFaceDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &RemoveWindFace, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(index) = base.wind_faces.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("wind face {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1991Diff { wind_faces: En1991WindFaceDelta::removal(&base.wind_faces, index), ..Default::default() })
}
