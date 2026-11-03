//! ↩️ `change-member-omega-wd` inverse: restores the previous `omega_wd`.

use crate::change_member_omega_wd::ChangeMemberOmegaWd;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberOmegaWd, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_omega_wd::ChangeMemberOmegaWd> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberOmegaWd, base: &En1998Snapshot) -> Option<ChangeMemberOmegaWd> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    let r1 = r0.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberOmegaWd { building_id: payload.building_id.clone(), id: payload.id.clone(), new_omega_wd: r1.omega_wd })
}
