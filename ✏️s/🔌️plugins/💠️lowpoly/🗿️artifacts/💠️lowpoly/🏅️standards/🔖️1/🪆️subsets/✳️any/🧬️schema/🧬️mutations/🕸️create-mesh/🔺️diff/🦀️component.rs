//! 🔺️ `create-mesh` — sparse diff construction: object patch on `mesh` (handle) and `mesh_workspace`
//! (content) together.

use super::mutation::CreateMesh;
use crate::artifacts::lowpoly::diff::diff_objects_patch;
use crate::artifacts::lowpoly::{LowpolyDiff, LowpolyObjectPatch, LowpolySnapshot};

//#region 🔖️Diff
pub fn diff(payload: &CreateMesh, _base: &LowpolySnapshot) -> LowpolyDiff {
    diff_objects_patch(payload.id.clone(), LowpolyObjectPatch {
        mesh: Some(Some(store::ArtifactChild::new(payload.child_id.clone(), payload.target.clone()))),
        mesh_workspace: Some(payload.mesh_workspace.clone()),
        ..LowpolyObjectPatch::default()
    })
}
//#endregion 🔖️Diff
