//! 🔺️ `delete-mesh` — sparse diff construction: clears `mesh` (handle) on the target object. The
//! live mesh content is not a document field at all any more (round 2 of this ticket's round-trip
//! law fix) — a live session's own `🖌️session::LowpolyScratch` cache drops/ignores its entry for
//! this object on its own terms, never through the document diff/apply pipeline.

use super::mutation::DeleteMesh;
use crate::artifacts::lowpoly::diff::diff_objects_patch;
use crate::artifacts::lowpoly::{LowpolyDiff, LowpolyObjectPatch, LowpolySnapshot};

//#region 🔖️Diff
pub fn diff(payload: &DeleteMesh, _base: &LowpolySnapshot) -> LowpolyDiff {
    diff_objects_patch(payload.id.clone(), LowpolyObjectPatch {
        mesh: Some(None),
        ..LowpolyObjectPatch::default()
    })
}
//#endregion 🔖️Diff
