//! 🔺️ Diff for `remove-floors`.
use crate::remove_floors::RemoveFloors;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
use semio_s_artifact_norm_en1991_diff::En1991FloorDelta;
pub fn diff(payload: &RemoveFloors, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if payload.index >= base.floors.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "Index out of range.", [payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1991Diff { floors: En1991FloorDelta::removal(&base.floors, payload.index), ..Default::default() })
}
