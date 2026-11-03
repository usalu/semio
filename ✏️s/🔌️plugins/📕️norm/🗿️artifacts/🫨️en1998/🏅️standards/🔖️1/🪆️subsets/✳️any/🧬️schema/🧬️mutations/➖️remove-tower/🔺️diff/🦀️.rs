//! ➖️ `remove-tower` diff: removes the addressed tower row.

use crate::remove_tower::RemoveTower;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998TowerDelta};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &RemoveTower, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(index) = base.towers.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("tower {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1998Diff { towers: En1998TowerDelta::removal(&base.towers, index), ..Default::default() })
}
