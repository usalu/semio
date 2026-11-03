//! ↩️ `change-member-concrete-grade-id` inverse: restores the previous `concrete_grade_id`.

use crate::change_member_concrete_grade_id::ChangeMemberConcreteGradeId;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberConcreteGradeId, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_concrete_grade_id::ChangeMemberConcreteGradeId> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberConcreteGradeId, base: &En1992Snapshot) -> Option<ChangeMemberConcreteGradeId> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    Some(ChangeMemberConcreteGradeId { member_id: payload.member_id.clone(), new_concrete_grade_id: r0.concrete_grade_id.clone() })
}
