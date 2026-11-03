//! ↩️ `change-member-min-dimension-m` inverse: restores the previous `min_dimension_m`.

use crate::change_member_min_dimension_m::ChangeMemberMinDimensionM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberMinDimensionM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_min_dimension_m::ChangeMemberMinDimensionM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberMinDimensionM, base: &En1998Snapshot) -> Option<ChangeMemberMinDimensionM> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    let r1 = r0.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberMinDimensionM { building_id: payload.building_id.clone(), id: payload.id.clone(), new_min_dimension_m: r1.min_dimension_m })
}
