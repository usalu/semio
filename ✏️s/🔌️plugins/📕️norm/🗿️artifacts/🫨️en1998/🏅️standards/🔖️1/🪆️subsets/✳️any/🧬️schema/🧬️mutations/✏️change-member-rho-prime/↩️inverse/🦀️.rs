//! ↩️ `change-member-rho-prime` inverse: restores the previous `rho_prime`.

use crate::change_member_rho_prime::ChangeMemberRhoPrime;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberRhoPrime, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_rho_prime::ChangeMemberRhoPrime> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberRhoPrime, base: &En1998Snapshot) -> Option<ChangeMemberRhoPrime> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    let r1 = r0.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberRhoPrime { building_id: payload.building_id.clone(), id: payload.id.clone(), new_rho_prime: r1.rho_prime })
}
