//! Diff for `insert-wind-faces`.
use super::InsertWindFaces;
use crate::artifact_schema::diff::En1991WindFacesList;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &InsertWindFaces, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if payload.index > base.wind_faces.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Index out of range.", Vec::<String>::new());
    }
    let mut values = base.wind_faces.clone();
    values.insert(payload.index, payload.item.clone());
    protocol::MutationOutcome::new(En1991Diff { wind_faces: Some(En1991WindFacesList { values }), ..Default::default() })
}
