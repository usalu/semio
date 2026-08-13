//! ↩️ `create-mesh` — undo restores whichever `mesh` HANDLE occupied the target object BEFORE this
//! create ran; missing object id ⇒ `Vec::new()`.
//!
//! ⚠️ The restored `CreateMesh.mesh_workspace` content is honestly empty, not the real prior
//! geometry: `base: &LowpolySnapshot` no longer carries live mesh JSON at all (round 2 of this
//! ticket's round-trip law fix — `LowpolyObject` has no content field to read it back from, only the
//! `mesh` handle, which is a one-way content hash). `diff::diff` above never reads `mesh_workspace`
//! either — the persisted document's `mesh` handle round-trips through undo/redo correctly on its
//! own via this inverse. What is lost is only the CONVENIENCE of a live kernel session (`🖌️session::LowpolyScratch`)
//! replaying real geometry from an undo of a create — that session's own cache is simply stale after
//! an undo (undo/redo bypass `ArtifactApp::handle` entirely — confirmed no app-level hook exists to
//! resync it, `store::os_store::ArtifactStore::dispatch_inner`'s `Undo`/`Redo` arms call no app code)
//! and needs real child-document resolution to close, same open gap flagged throughout this ticket.
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
            mesh_workspace: String::new(),
        })],
        None => vec![LowpolyMutation::DeleteMesh(delete_mesh::mutation::DeleteMesh { id: payload.id.clone() })],
    }
}
//#endregion 🔖️Inverse
