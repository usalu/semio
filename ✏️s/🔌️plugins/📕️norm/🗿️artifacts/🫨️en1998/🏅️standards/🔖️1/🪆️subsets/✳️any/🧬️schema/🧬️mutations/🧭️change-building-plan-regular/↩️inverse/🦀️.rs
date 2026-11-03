//! ↩️ `change-building-plan-regular` inverse: restores the previous `plan_regular`.

use crate::change_building_plan_regular::ChangeBuildingPlanRegular;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBuildingPlanRegular, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_building_plan_regular::ChangeBuildingPlanRegular> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBuildingPlanRegular, base: &En1998Snapshot) -> Option<ChangeBuildingPlanRegular> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBuildingPlanRegular { id: payload.id.clone(), new_plan_regular: r0.plan_regular })
}
