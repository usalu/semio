//! ↩️ `change-member-rho` inverse: restores the previous `rho`.

use crate::change_member_rho::ChangeMemberRho;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberRho, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_rho::ChangeMemberRho> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberRho, base: &En1998Snapshot) -> Option<ChangeMemberRho> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    let r1 = r0.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberRho { building_id: payload.building_id.clone(), id: payload.id.clone(), new_rho: r1.rho })
}
