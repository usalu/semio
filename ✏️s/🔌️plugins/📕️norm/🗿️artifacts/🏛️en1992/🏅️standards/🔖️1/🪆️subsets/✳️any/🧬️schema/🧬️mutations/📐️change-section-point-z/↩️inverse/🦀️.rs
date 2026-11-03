//! ↩️ `change-section-point-z` inverse: restores the previous `z`.

use crate::change_section_point_z::ChangeSectionPointZ;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeSectionPointZ, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_section_point_z::ChangeSectionPointZ> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSectionPointZ, base: &En1992Snapshot) -> Option<ChangeSectionPointZ> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let r1 = r0.section_points.iter().find(|row| row.id == payload.section_point_id)?;
    Some(ChangeSectionPointZ { member_id: payload.member_id.clone(), section_point_id: payload.section_point_id.clone(), new_z: r1.z })
}
