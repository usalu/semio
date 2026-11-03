//! ✏️ `change-tower-is-chimney` diff: patches `is_chimney` of the addressed tower row.

use crate::change_tower_is_chimney::ChangeTowerIsChimney;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998TowerDelta, En1998TowerPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeTowerIsChimney, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.towers.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("tower {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.is_chimney == payload.new_is_chimney {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The tower already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { towers: En1998TowerDelta::modification(&payload.id, En1998TowerPatch { is_chimney: Some(payload.new_is_chimney), ..Default::default() }), ..Default::default() })
}
