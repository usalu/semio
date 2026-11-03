//! ✏️ `change-building-plan-env-gap-ratio` diff: patches `plan_envelope_gap_ratio` of the addressed building row.

use crate::change_building_plan_env_gap_ratio::ChangeBuildingPlanEnvGapRatio;
use semio_s_artifact_norm_en1998_diff::{En1998BuildingDelta, En1998BuildingPatch, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeBuildingPlanEnvGapRatio, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_plan_envelope_gap_ratio.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.buildings.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("building {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.plan_envelope_gap_ratio == payload.new_plan_envelope_gap_ratio {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The building already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { buildings: En1998BuildingDelta::modification(&payload.id, En1998BuildingPatch { plan_envelope_gap_ratio: Some(payload.new_plan_envelope_gap_ratio), ..Default::default() }), ..Default::default() })
}
