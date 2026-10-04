//! 🔺️ Diff for `change-wind-face-assumed-wp`.
use super::ChangeWindFaceAssumedWp;
use crate::artifact_schema::diff::En1991WindFacesList;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeWindFaceAssumedWp, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if payload.index >= base.wind_faces.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "Index out of range.", [payload.index.to_string()]);
    }
    if base.wind_faces[payload.index].assumed_wp == payload.new_assumed_wp {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    let mut values = base.wind_faces.clone();
    values[payload.index].assumed_wp = payload.new_assumed_wp;
    protocol::MutationOutcome::new(En1991Diff { wind_faces: Some(En1991WindFacesList { values }), ..Default::default() })
}
