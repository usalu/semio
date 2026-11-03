//! ➖️ `remove-floor` diff: removes the addressed floor row.

use crate::remove_floor::RemoveFloor;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991FloorDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &RemoveFloor, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(index) = base.floors.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("floor {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1991Diff { floors: En1991FloorDelta::removal(&base.floors, index), ..Default::default() })
}
