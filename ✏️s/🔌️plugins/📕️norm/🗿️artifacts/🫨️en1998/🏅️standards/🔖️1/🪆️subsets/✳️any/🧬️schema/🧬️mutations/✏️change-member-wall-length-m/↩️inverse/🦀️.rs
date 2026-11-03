//! ↩️ `change-member-wall-length-m` inverse: restores the previous `wall_length_m`.

use crate::change_member_wall_length_m::ChangeMemberWallLengthM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberWallLengthM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_wall_length_m::ChangeMemberWallLengthM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberWallLengthM, base: &En1998Snapshot) -> Option<ChangeMemberWallLengthM> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    let r1 = r0.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberWallLengthM { building_id: payload.building_id.clone(), id: payload.id.clone(), new_wall_length_m: r1.wall_length_m })
}
