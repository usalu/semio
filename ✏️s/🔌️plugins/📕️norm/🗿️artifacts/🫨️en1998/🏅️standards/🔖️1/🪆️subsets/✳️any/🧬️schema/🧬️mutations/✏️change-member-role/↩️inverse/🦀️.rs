//! ↩️ `change-member-role` inverse: restores the previous `role`.

use crate::change_member_role::ChangeMemberRole;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberRole, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_role::ChangeMemberRole> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberRole, base: &En1998Snapshot) -> Option<ChangeMemberRole> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    let r1 = r0.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberRole { building_id: payload.building_id.clone(), id: payload.id.clone(), new_role: r1.role })
}
