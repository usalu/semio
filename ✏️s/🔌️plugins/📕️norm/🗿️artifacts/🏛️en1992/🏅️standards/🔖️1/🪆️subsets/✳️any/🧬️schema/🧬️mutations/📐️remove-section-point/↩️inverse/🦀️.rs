//! ↩️ `remove-section-point` inverse: inserts the removed row back at its position.

use crate::remove_section_point::RemoveSectionPoint;
use crate::insert_section_point::InsertSectionPoint;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &RemoveSectionPoint, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_section_point::InsertSectionPoint> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveSectionPoint, base: &En1992Snapshot) -> Option<InsertSectionPoint> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let index = payload.index;
    let row = r0.section_points.get(index)?;
    Some(InsertSectionPoint { member_id: payload.member_id.clone(), index, section_point: row.clone() })
}
