//! ✏️ `change-building-col-moment-sum-nm` diff: patches `column_moment_sum_nm` of the addressed building row.

use crate::change_building_col_moment_sum_nm::ChangeBuildingColMomentSumNm;
use semio_s_artifact_norm_en1998_diff::{En1998BuildingDelta, En1998BuildingPatch, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeBuildingColMomentSumNm, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_column_moment_sum_nm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.buildings.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("building {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.column_moment_sum_nm == payload.new_column_moment_sum_nm {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The building already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { buildings: En1998BuildingDelta::modification(&payload.id, En1998BuildingPatch { column_moment_sum_nm: Some(payload.new_column_moment_sum_nm), ..Default::default() }), ..Default::default() })
}
