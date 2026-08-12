//! 🔺️ `replace-object-mesh` — sparse diff construction: one-field object patch on `mesh_json`.

use super::mutation::ReplaceObjectMesh;
use crate::artifacts::lowpoly::diff::diff_objects_patch;
use crate::artifacts::lowpoly::{LowpolyDiff, LowpolyObjectPatch, LowpolySnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ReplaceObjectMesh, _base: &LowpolySnapshot) -> LowpolyDiff {
    diff_objects_patch(payload.id.clone(), LowpolyObjectPatch { mesh_json: Some(payload.new_mesh_json.clone()), ..LowpolyObjectPatch::default() })
}
//#endregion 🔖️Diff
