//! ↩️ `delete-mesh` — undo is `create-mesh` with the escrowed handle+workspace from BASE; empty
//! when absent or the object doesn't exist.

use super::mutation::DeleteMesh;
use crate::artifacts::lowpoly::mutations::create_mesh;
use crate::artifacts::lowpoly::{LowpolyMutation, LowpolySnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteMesh, base: &LowpolySnapshot) -> Vec<LowpolyMutation> {
    let Some(object) = base.objects.iter().find(|object| object.id == payload.id) else {
        return Vec::new();
    };
    match &object.mesh {
        Some(existing) => vec![LowpolyMutation::CreateMesh(create_mesh::mutation::CreateMesh {
            id: payload.id.clone(),
            child_id: existing.child_id.clone(),
            target: existing.target.clone(),
            mesh_workspace: object.mesh_workspace.clone(),
        })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
