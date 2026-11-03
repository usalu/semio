//! ↩️ `insert-section-point` inverse: removes the inserted row again.

use crate::insert_section_point::InsertSectionPoint;
use crate::remove_section_point::RemoveSectionPoint;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &InsertSectionPoint, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_section_point::RemoveSectionPoint> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertSectionPoint, base: &En1992Snapshot) -> Option<RemoveSectionPoint> {
    let _r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let index = payload.index;
    Some(RemoveSectionPoint { member_id: payload.member_id.clone(), index })
}
