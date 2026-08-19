//! 🔺️ `create-mesh` — sparse diff construction: object patch on `mesh` (handle) only. `mesh_workspace`
//! (content) is carried on the `CreateMesh` payload itself as event-log data for the originating
//! session's own `🖌️session::LowpolyScratch` cache to replay — it is NOT a `LowpolyObject`/
//! `LowpolyObjectPatch` field at all (round 2 of this ticket's round-trip law fix) and never touches
//! the persisted document.

//! Error `target-missing` when the owning object is absent; overwrite-aware (no `duplicate-id`
//! check) — replacing an already-present mesh handle is deliberate, per this triad's mutation doc.

use super::mutation::CreateMesh;
use crate::artifacts::lowpoly::diff::diff_objects_patch;
use crate::artifacts::lowpoly::{LowpolyDiff, LowpolyObjectPatch, LowpolySnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &CreateMesh, base: &LowpolySnapshot) -> protocol::MutationOutcome<LowpolyDiff> {
    if !base.objects.iter().any(|object| object.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Object \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(diff_objects_patch(payload.id.clone(), LowpolyObjectPatch {
        mesh: Some(Some(store::ArtifactChild::new(payload.child_id.clone(), payload.target.clone()))),
        ..LowpolyObjectPatch::default()
    }))
}
//#endregion 🔖️Diff
