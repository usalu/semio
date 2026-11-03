//! 🔺️ `change-storey-building-height` diff: patches `building_height` of the addressed storey row.

use crate::change_storey_building_height::ChangeStoreyBuildingHeight;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993StoreyDelta, En1993StoreyPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeStoreyBuildingHeight, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_building_height.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.storeys.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "storey", Vec::<String>::new());
    };
    if row.building_height == payload.new_building_height {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The storey already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { storeys: En1993StoreyDelta::modification(payload.id.clone(), En1993StoreyPatch { building_height: Some(payload.new_building_height), ..Default::default() }), ..Default::default() })
}
