use crate::change_member_stirrup_spacing::ChangeMemberStirrupSpacing;

use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberStirrupSpacing, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_stirrup_spacing::ChangeMemberStirrupSpacing> {
    Ok((|| {
    let Some(m) = base.members.iter().find(|m| m.id == payload.member_id) else { return vec![]; };
    let Some(s) = m.stirrups.as_ref() else { return vec![]; };
    vec![Op::from(ChangeMemberStirrupSpacing { member_id: payload.member_id.clone(), new_spacing: s.spacing })]

    })())
}
