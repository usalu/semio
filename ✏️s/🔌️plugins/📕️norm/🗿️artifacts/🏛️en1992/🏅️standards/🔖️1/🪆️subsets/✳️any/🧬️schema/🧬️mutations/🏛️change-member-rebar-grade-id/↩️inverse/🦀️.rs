//! ↩️ `change-member-rebar-grade-id` inverse: restores the previous `reinforcement_grade_id`.

use crate::change_member_rebar_grade_id::ChangeMemberRebarGradeId;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberRebarGradeId, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_rebar_grade_id::ChangeMemberRebarGradeId> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberRebarGradeId, base: &En1992Snapshot) -> Option<ChangeMemberRebarGradeId> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    Some(ChangeMemberRebarGradeId { member_id: payload.member_id.clone(), new_reinforcement_grade_id: r0.reinforcement_grade_id.clone() })
}
