use crate::remove_floor::RemoveFloor;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::En1995FloorDelta;
pub fn diff(payload: &RemoveFloor, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if payload.index >= base.floors.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Floor index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1995Diff { floors: En1995FloorDelta::removal(&base.floors, payload.index), ..Default::default() })
}
