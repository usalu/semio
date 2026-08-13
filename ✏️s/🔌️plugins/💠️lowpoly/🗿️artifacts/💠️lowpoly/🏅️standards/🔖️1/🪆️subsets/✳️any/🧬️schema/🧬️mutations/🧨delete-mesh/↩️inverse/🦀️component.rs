//! ↩️ `delete-mesh` — undo is `create-mesh` with the escrowed `mesh` HANDLE from BASE; empty when
//! absent or the object doesn't exist.
//!
//! ⚠️ Same honest gap as `🕸️create-mesh/↩️inverse`: `base: &LowpolySnapshot` no longer carries live
//! mesh JSON (round 2 of this ticket's round-trip law fix), so the restored `CreateMesh.mesh_workspace`
//! is empty, not the real prior geometry. The persisted `mesh` handle still round-trips correctly —
//! only a live session's own `🖌️session::LowpolyScratch` replay convenience is affected, and only
//! for an undo of a delete specifically.

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
            mesh_workspace: String::new(),
        })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
