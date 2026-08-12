//! 🔺️ `delete-mesh` — sparse diff construction: clears `mesh` (handle) and `mesh_workspace`
//! (content) together on the target object.

use super::mutation::DeleteMesh;
use crate::artifacts::lowpoly::diff::diff_objects_patch;
use crate::artifacts::lowpoly::{LowpolyDiff, LowpolyObjectPatch, LowpolySnapshot};

//#region 🔖️Diff
pub fn diff(payload: &DeleteMesh, _base: &LowpolySnapshot) -> LowpolyDiff {
    diff_objects_patch(payload.id.clone(), LowpolyObjectPatch {
        mesh: Some(None),
        mesh_workspace: Some(String::new()),
        ..LowpolyObjectPatch::default()
    })
}
//#endregion 🔖️Diff
