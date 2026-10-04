//! 🕸️ Mesh operations preserve independent source literals and the complete typed managed owner.

use super::DeleteMesh;
use crate::diff::diff_objects_patch;
use crate::{LowpolyDiff, LowpolyObjectPatch, LowpolySnapshot};

//#region 🔖️Diff
pub fn diff(payload: &DeleteMesh, base: &LowpolySnapshot) -> protocol::MutationOutcome<LowpolyDiff> {
    let Some(object) = base.objects.iter().find(|object| object.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Object \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if object.mesh.is_none() {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Object \"{}\" has no mesh to delete.", payload.id));
    }
    protocol::MutationOutcome::new(diff_objects_patch(payload.id.clone(), LowpolyObjectPatch { mesh: Some(None), mesh_state:Some(None),mesh_content:Some(String::new()), ..LowpolyObjectPatch::default() }))
}
//#endregion 🔖️Diff
