//! ✏️ `change-retaining-wall-permeability-ms` diff — patches the one field of the addressed row; a missing target is a `mutation.target-missing`.

use crate::change_retaining_wall_permeability_m_s::ChangeRetainingWallPermeabilityMS;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998RetainingWallDelta, En1998RetainingWallPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeRetainingWallPermeabilityMS, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(row) = base.retaining_walls.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "retaining wall", Vec::<String>::new());
    };
    let patch = En1998RetainingWallPatch { permeability_m_s: Some(payload.new_permeability_m_s), ..Default::default() };
    protocol::MutationOutcome::new(En1998Diff { retaining_walls: En1998RetainingWallDelta::modification(&row.id, patch), ..Default::default() })
}
