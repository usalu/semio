use crate::insert_floor::InsertFloor;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::En1995FloorDelta;
pub fn diff(payload: &InsertFloor, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if base.floors.iter().any(|existing| existing.id == payload.floor.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Floor id {} already exists.", payload.floor.id), [payload.floor.id.clone()]);
    }
    let at = payload.index.min(base.floors.len());
    protocol::MutationOutcome::new(En1995Diff { floors: En1995FloorDelta::insertion(at, payload.floor.clone()), ..Default::default() })
}
