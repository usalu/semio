//! ↩️ `change-member-effective-stiffness-basis` inverse: restores the previous `effective_stiffness_basis`.

use crate::change_member_effective_stiffness_basis::ChangeMemberEffectiveStiffnessBasis;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberEffectiveStiffnessBasis, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_effective_stiffness_basis::ChangeMemberEffectiveStiffnessBasis> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberEffectiveStiffnessBasis, base: &En1998Snapshot) -> Option<ChangeMemberEffectiveStiffnessBasis> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    let r1 = r0.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberEffectiveStiffnessBasis { building_id: payload.building_id.clone(), id: payload.id.clone(), new_effective_stiffness_basis: r1.effective_stiffness_basis })
}
