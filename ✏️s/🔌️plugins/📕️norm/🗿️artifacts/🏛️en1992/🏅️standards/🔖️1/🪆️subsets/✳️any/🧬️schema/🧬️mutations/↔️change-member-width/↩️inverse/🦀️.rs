use crate::change_member_width::ChangeMemberWidth;

use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberWidth, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_width::ChangeMemberWidth> {
    Ok((|| {
    let Some(m) = base.members.iter().find(|m| m.id == payload.member_id) else { return vec![]; };
    vec![Op::from(ChangeMemberWidth { member_id: payload.member_id.clone(), new_value: m.width })]

    })())
}
