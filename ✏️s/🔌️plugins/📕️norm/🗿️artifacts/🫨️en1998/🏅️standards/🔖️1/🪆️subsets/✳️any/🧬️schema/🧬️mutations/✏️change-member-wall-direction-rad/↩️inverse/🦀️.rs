//! ↩️ `change-member-wall-direction-rad` inverse: restores the previous `wall_direction_rad`.

use crate::change_member_wall_direction_rad::ChangeMemberWallDirectionRad;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberWallDirectionRad, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_wall_direction_rad::ChangeMemberWallDirectionRad> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberWallDirectionRad, base: &En1998Snapshot) -> Option<ChangeMemberWallDirectionRad> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    let r1 = r0.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberWallDirectionRad { building_id: payload.building_id.clone(), id: payload.id.clone(), new_wall_direction_rad: r1.wall_direction_rad })
}
