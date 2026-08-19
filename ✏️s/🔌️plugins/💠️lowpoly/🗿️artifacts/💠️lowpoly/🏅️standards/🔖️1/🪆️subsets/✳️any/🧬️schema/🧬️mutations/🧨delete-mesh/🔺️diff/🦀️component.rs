//! 🔺️ `delete-mesh` — sparse diff construction: clears `mesh` (handle) on the target object. The
//! live mesh content is not a document field at all any more (round 2 of this ticket's round-trip
//! law fix) — a live session's own `🖌️session::LowpolyScratch` cache drops/ignores its entry for
//! this object on its own terms, never through the document diff/apply pipeline.

//! Error `target-missing` when the object is absent; Warning `no-op` when the mesh slot is already
//! empty — treated as a `clear`-style slot (idempotent per this triad's mutation doc), not a
//! collection `delete`.

use super::mutation::DeleteMesh;
use crate::artifacts::lowpoly::diff::diff_objects_patch;
use crate::artifacts::lowpoly::{LowpolyDiff, LowpolyObjectPatch, LowpolySnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &DeleteMesh, base: &LowpolySnapshot) -> protocol::MutationOutcome<LowpolyDiff> {
    let Some(object) = base.objects.iter().find(|object| object.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Object \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if object.mesh.is_none() {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Object \"{}\" has no mesh to delete.", payload.id));
    }
    protocol::MutationOutcome::new(diff_objects_patch(payload.id.clone(), LowpolyObjectPatch {
        mesh: Some(None),
        ..LowpolyObjectPatch::default()
    }))
}
//#endregion 🔖️Diff
