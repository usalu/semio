//! ↩️ upsert inverse — restore prior entity or remove inserted one.
use crate::update_member_properties::UpdateMemberProperties;
use crate::remove_member;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &UpdateMemberProperties, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::update_member_properties::UpdateMemberProperties> + From<crate::remove_member::RemoveMember> {
    Ok((|| {
    if let Some(prior) = base.members.iter().find(|x| x.id == payload.member.id) {
        vec![Op::from(UpdateMemberProperties { member: prior.clone() })]
    } else {
        vec![Op::from(remove_member::RemoveMember { index: base.members.len() })]
    }

    })())
}
