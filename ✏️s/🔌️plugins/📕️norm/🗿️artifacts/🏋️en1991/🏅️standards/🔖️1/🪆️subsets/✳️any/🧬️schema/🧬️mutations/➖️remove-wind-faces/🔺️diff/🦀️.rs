//! 🔺️ Diff for `remove-wind-faces`.
use super::RemoveWindFaces;
use crate::{En1991Diff, En1991Snapshot};
use crate::diff::{En1991WindFaceDelta};
pub fn diff(payload: &RemoveWindFaces, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if payload.index >= base.wind_faces.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "Index out of range.", [payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1991Diff { wind_faces: En1991WindFaceDelta::removal(&base.wind_faces[payload.index].id), ..Default::default() })
}
