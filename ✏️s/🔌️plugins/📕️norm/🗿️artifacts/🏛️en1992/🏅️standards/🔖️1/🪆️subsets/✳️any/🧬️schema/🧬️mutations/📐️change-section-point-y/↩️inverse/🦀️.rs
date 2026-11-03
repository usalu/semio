//! ↩️ `change-section-point-y` inverse: restores the previous `y`.

use crate::change_section_point_y::ChangeSectionPointY;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeSectionPointY, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_section_point_y::ChangeSectionPointY> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSectionPointY, base: &En1992Snapshot) -> Option<ChangeSectionPointY> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let r1 = r0.section_points.iter().find(|row| row.id == payload.section_point_id)?;
    Some(ChangeSectionPointY { member_id: payload.member_id.clone(), section_point_id: payload.section_point_id.clone(), new_y: r1.y })
}
