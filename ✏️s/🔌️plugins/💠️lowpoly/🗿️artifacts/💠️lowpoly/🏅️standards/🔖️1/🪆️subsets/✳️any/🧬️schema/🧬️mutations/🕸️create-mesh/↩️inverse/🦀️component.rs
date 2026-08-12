//! ↩️ `create-mesh` — undo restores whichever handle+workspace occupied the target object BEFORE
//! this create ran; missing object id ⇒ `Vec::new()`.

use super::mutation::CreateMesh;
use crate::artifacts::lowpoly::mutations::delete_mesh;
use crate::artifacts::lowpoly::{LowpolyMutation, LowpolySnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &CreateMesh, base: &LowpolySnapshot) -> Vec<LowpolyMutation> {
    let Some(object) = base.objects.iter().find(|object| object.id == payload.id) else {
        return Vec::new();
    };
    match &object.mesh {
        Some(existing) => vec![LowpolyMutation::CreateMesh(CreateMesh {
            id: payload.id.clone(),
            child_id: existing.child_id.clone(),
            target: existing.target.clone(),
            mesh_workspace: object.mesh_workspace.clone(),
        })],
        None => vec![LowpolyMutation::DeleteMesh(delete_mesh::mutation::DeleteMesh { id: payload.id.clone() })],
    }
}
//#endregion 🔖️Inverse
