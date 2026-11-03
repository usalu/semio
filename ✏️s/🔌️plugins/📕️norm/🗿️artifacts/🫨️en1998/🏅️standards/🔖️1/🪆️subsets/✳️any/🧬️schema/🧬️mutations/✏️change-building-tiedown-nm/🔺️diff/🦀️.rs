//! ✏️ `change-building-tiedown-nm` diff: patches `tie_down_moment_nm` of the addressed building row.

use crate::change_building_tiedown_nm::ChangeBuildingTiedownNm;
use semio_s_artifact_norm_en1998_diff::{En1998BuildingDelta, En1998BuildingPatch, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeBuildingTiedownNm, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_tie_down_moment_nm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.buildings.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("building {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.tie_down_moment_nm == payload.new_tie_down_moment_nm {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The building already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { buildings: En1998BuildingDelta::modification(&payload.id, En1998BuildingPatch { tie_down_moment_nm: Some(payload.new_tie_down_moment_nm), ..Default::default() }), ..Default::default() })
}
