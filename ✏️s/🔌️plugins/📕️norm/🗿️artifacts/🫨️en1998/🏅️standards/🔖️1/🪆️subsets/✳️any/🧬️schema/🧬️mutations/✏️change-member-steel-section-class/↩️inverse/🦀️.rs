//! ↩️ `change-member-steel-section-class` inverse: restores the previous `steel_section_class`.

use crate::change_member_steel_section_class::ChangeMemberSteelSectionClass;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberSteelSectionClass, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_steel_section_class::ChangeMemberSteelSectionClass> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberSteelSectionClass, base: &En1998Snapshot) -> Option<ChangeMemberSteelSectionClass> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    let r1 = r0.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberSteelSectionClass { building_id: payload.building_id.clone(), id: payload.id.clone(), new_steel_section_class: r1.steel_section_class })
}
