//! ↩️ `change-member-wall-storey-id` inverse: restores the previous `wall_storey_id`.

use crate::change_member_wall_storey_id::ChangeMemberWallStoreyId;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberWallStoreyId, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_wall_storey_id::ChangeMemberWallStoreyId> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberWallStoreyId, base: &En1998Snapshot) -> Option<ChangeMemberWallStoreyId> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    let r1 = r0.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberWallStoreyId { building_id: payload.building_id.clone(), id: payload.id.clone(), new_wall_storey_id: r1.wall_storey_id.clone() })
}
