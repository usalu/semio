//! Inverse for `change-building-plan-regular`.
use crate::change_building_plan_regular::ChangeBuildingPlanRegular;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBuildingPlanRegular, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_building_plan_regular::ChangeBuildingPlanRegular> {
    Ok((|| {
    match base.buildings.get(payload.building_index) {
        Some(b) => vec![Op::from(ChangeBuildingPlanRegular { building_index: payload.building_index, new_plan_regular: b.plan_regular })],
        None => Vec::new(),
    }

    })())
}
