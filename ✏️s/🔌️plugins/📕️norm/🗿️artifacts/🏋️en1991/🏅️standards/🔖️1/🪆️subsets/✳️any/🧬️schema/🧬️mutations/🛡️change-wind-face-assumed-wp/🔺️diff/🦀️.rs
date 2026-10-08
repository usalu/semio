//! 🔺️ Diff for `change-wind-face-assumed-wp`.
use super::ChangeWindFaceAssumedWp;
use crate::{En1991Diff, En1991Snapshot};
use crate::diff::{En1991WindFaceDelta, En1991WindFacePatch};
pub fn diff(payload: &ChangeWindFaceAssumedWp, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if payload.index >= base.wind_faces.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "Index out of range.", [payload.index.to_string()]);
    }
    if base.wind_faces[payload.index].assumed_wp == payload.new_assumed_wp {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    let wind_face = &base.wind_faces[payload.index];
    protocol::MutationOutcome::new(En1991Diff { wind_faces: En1991WindFaceDelta::modification(&wind_face.id, En1991WindFacePatch { assumed_wp: Some(payload.new_assumed_wp), ..Default::default() }), ..Default::default() })
}
