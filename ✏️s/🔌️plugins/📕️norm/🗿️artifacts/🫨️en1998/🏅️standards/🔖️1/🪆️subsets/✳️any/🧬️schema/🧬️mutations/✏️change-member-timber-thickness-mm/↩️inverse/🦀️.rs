//! ↩️ `change-member-timber-thickness-mm` inverse: restores the previous `timber_thickness_mm`.

use crate::change_member_timber_thickness_mm::ChangeMemberTimberThicknessMm;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberTimberThicknessMm, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_timber_thickness_mm::ChangeMemberTimberThicknessMm> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberTimberThicknessMm, base: &En1998Snapshot) -> Option<ChangeMemberTimberThicknessMm> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    let r1 = r0.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberTimberThicknessMm { building_id: payload.building_id.clone(), id: payload.id.clone(), new_timber_thickness_mm: r1.timber_thickness_mm })
}
