//! ↩️ `change-member-member-alpha-cr` inverse: restores the previous `member_alpha_cr`.

use crate::change_member_member_alpha_cr::ChangeMemberMemberAlphaCr;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberMemberAlphaCr, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_member_alpha_cr::ChangeMemberMemberAlphaCr> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberMemberAlphaCr, base: &En1990Snapshot) -> Option<ChangeMemberMemberAlphaCr> {
    let r0 = base.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberMemberAlphaCr { id: payload.id.clone(), new_member_alpha_cr: r0.member_alpha_cr })
}
