//! ➖️ `remove-building` diff — removes the row at the index, guarded by the row's own id; an index past the collection's end is a `mutation.target-missing`.

use crate::remove_building::RemoveBuilding;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998BuildingDelta};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &RemoveBuilding, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if payload.index >= base.buildings.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("building #{}", payload.index), [payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1998Diff { buildings: En1998BuildingDelta::removal(&base.buildings, payload.index), ..Default::default() })
}
