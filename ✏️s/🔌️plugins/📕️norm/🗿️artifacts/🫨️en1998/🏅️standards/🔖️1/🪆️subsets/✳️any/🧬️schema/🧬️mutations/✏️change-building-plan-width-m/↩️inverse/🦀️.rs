//! ↩️ `change-building-plan-width-m` inverse: restores the previous `plan_width_m`.

use crate::change_building_plan_width_m::ChangeBuildingPlanWidthM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBuildingPlanWidthM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_building_plan_width_m::ChangeBuildingPlanWidthM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBuildingPlanWidthM, base: &En1998Snapshot) -> Option<ChangeBuildingPlanWidthM> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBuildingPlanWidthM { id: payload.id.clone(), new_plan_width_m: r0.plan_width_m })
}
