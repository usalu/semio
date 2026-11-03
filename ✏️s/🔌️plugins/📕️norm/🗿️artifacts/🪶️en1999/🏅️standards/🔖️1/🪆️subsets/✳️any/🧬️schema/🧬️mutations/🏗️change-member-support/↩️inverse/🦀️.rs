//! ↩️ `change-member-support` inverse: restores the previous `support`.

use crate::change_member_support::ChangeMemberSupport;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberSupport, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_support::ChangeMemberSupport> {
    Ok(match base.members.iter().find(|row| row.id == payload.member_id) {
        Some(row) => vec![Op::from(ChangeMemberSupport { member_id: payload.member_id.clone(), new_support: row.support })],
        None => Vec::new(),
    })
}
