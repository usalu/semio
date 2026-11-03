//! ↩️ `change-building-plan-env-gap-ratio` inverse: restores the previous `plan_envelope_gap_ratio`.

use crate::change_building_plan_env_gap_ratio::ChangeBuildingPlanEnvGapRatio;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBuildingPlanEnvGapRatio, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_building_plan_env_gap_ratio::ChangeBuildingPlanEnvGapRatio> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBuildingPlanEnvGapRatio, base: &En1998Snapshot) -> Option<ChangeBuildingPlanEnvGapRatio> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBuildingPlanEnvGapRatio { id: payload.id.clone(), new_plan_envelope_gap_ratio: r0.plan_envelope_gap_ratio })
}
