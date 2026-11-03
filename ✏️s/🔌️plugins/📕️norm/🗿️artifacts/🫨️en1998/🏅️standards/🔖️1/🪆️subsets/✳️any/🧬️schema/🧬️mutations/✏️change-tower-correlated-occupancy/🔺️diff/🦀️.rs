//! ✏️ `change-tower-correlated-occupancy` diff: patches `correlated_occupancy` of the addressed tower row.

use crate::change_tower_correlated_occupancy::ChangeTowerCorrelatedOccupancy;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998TowerDelta, En1998TowerPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeTowerCorrelatedOccupancy, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.towers.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("tower {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.correlated_occupancy == payload.new_correlated_occupancy {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The tower already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { towers: En1998TowerDelta::modification(&payload.id, En1998TowerPatch { correlated_occupancy: Some(payload.new_correlated_occupancy), ..Default::default() }), ..Default::default() })
}
