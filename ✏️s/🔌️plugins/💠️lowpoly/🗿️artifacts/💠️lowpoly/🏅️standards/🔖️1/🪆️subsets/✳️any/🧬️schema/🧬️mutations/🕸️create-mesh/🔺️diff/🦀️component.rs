//! 🔺️ `create-mesh` — sparse diff construction: object patch on `mesh` (handle) only. `mesh_workspace`
//! (content) is carried on the `CreateMesh` payload itself as event-log data for the originating
//! session's own `🖌️session::LowpolyScratch` cache to replay — it is NOT a `LowpolyObject`/
//! `LowpolyObjectPatch` field at all (round 2 of this ticket's round-trip law fix) and never touches
//! the persisted document.

use super::mutation::CreateMesh;
use crate::artifacts::lowpoly::diff::diff_objects_patch;
use crate::artifacts::lowpoly::{LowpolyDiff, LowpolyObjectPatch, LowpolySnapshot};

//#region 🔖️Diff
pub fn diff(payload: &CreateMesh, _base: &LowpolySnapshot) -> LowpolyDiff {
    diff_objects_patch(payload.id.clone(), LowpolyObjectPatch {
        mesh: Some(Some(store::ArtifactChild::new(payload.child_id.clone(), payload.target.clone()))),
        ..LowpolyObjectPatch::default()
    })
}
//#endregion 🔖️Diff
