//! ↩️ `change-member-restrained-ltb` inverse: restores the previous `restrained_ltb`.

use crate::change_member_restrained_ltb::ChangeMemberRestrainedLtb;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberRestrainedLtb, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_restrained_ltb::ChangeMemberRestrainedLtb> {
    Ok(match base.members.iter().find(|row| row.id == payload.member_id) {
        Some(row) => vec![Op::from(ChangeMemberRestrainedLtb { member_id: payload.member_id.clone(), new_restrained_ltb: row.restrained_ltb })],
        None => Vec::new(),
    })
}
