//! 🕸️ Mesh operations preserve independent source literals and the complete typed managed owner.

use super::CreateMesh;
use crate::diff::diff_objects_patch;
use crate::{LowpolyDiff, LowpolyObjectPatch, LowpolySnapshot};

//#region 🔖️Diff
pub fn diff(payload: &CreateMesh, base: &LowpolySnapshot) -> protocol::MutationOutcome<LowpolyDiff> {
    if !base.objects.iter().any(|object| object.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Object \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(diff_objects_patch(payload.id.clone(), LowpolyObjectPatch { mesh: Some(Some(store::ArtifactChild::new(payload.child_id.clone(), payload.target.clone()))), mesh_content: Some(payload.mesh_workspace.clone()), mesh_state:Some(payload.mesh_state.clone()), ..LowpolyObjectPatch::default() }))
}
//#endregion 🔖️Diff
