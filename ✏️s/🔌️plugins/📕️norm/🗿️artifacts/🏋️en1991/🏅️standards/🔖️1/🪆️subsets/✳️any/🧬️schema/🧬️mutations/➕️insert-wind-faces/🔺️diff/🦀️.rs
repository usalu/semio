//! 🔺️ Diff for `insert-wind-faces`.
use super::InsertWindFaces;
use crate::{En1991Diff, En1991Snapshot};
use crate::diff::{En1991WindFaceDelta};
pub fn diff(payload: &InsertWindFaces, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if payload.index > base.wind_faces.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "Index out of range.", [payload.index.to_string()]);
    }
    if base.wind_faces.iter().any(|existing| existing.id == payload.item.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Row id {} already exists.", payload.item.id), [payload.item.id.clone()]);
    }
    protocol::MutationOutcome::new(En1991Diff { wind_faces: En1991WindFaceDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
