//! ➖️ `remove-building` diff: removes the addressed building row.

use crate::remove_building::RemoveBuilding;
use semio_s_artifact_norm_en1998_diff::{En1998BuildingDelta, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &RemoveBuilding, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(index) = base.buildings.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("building {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1998Diff { buildings: En1998BuildingDelta::removal(&base.buildings, index), ..Default::default() })
}
