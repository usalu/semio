use crate::change_member_fire_duration::ChangeMemberFireDuration;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeMemberFireDuration, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_fire_duration::ChangeMemberFireDuration> {
    Ok((|| {
    let Some(item) = base.members.iter().find(|item| item.id == payload.member_id) else { return Vec::new(); };
    vec![Op::from(ChangeMemberFireDuration { member_id: payload.member_id.clone(), new_value: item.fire_duration_s })]

    })())
}
