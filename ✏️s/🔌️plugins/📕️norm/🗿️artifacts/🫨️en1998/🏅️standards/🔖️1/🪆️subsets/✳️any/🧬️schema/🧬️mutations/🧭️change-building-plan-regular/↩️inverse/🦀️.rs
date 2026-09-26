//! Inverse for `change-building-plan-regular`.
use super::ChangeBuildingPlanRegular;
use crate::{En1998Mutation, En1998Snapshot};

pub fn inverse(payload: &ChangeBuildingPlanRegular, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    match base.buildings.get(payload.building_index) {
        Some(b) => vec![En1998Mutation::ChangeBuildingPlanRegular(ChangeBuildingPlanRegular { building_index: payload.building_index, new_plan_regular: b.plan_regular })],
        None => Vec::new(),
    }
}
