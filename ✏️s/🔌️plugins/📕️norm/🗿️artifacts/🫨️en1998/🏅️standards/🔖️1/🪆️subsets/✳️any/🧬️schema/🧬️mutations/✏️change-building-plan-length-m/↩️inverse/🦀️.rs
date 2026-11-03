//! ↩️ `change-building-plan-length-m` inverse: restores the previous `plan_length_m`.

use crate::change_building_plan_length_m::ChangeBuildingPlanLengthM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBuildingPlanLengthM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_building_plan_length_m::ChangeBuildingPlanLengthM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBuildingPlanLengthM, base: &En1998Snapshot) -> Option<ChangeBuildingPlanLengthM> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBuildingPlanLengthM { id: payload.id.clone(), new_plan_length_m: r0.plan_length_m })
}
