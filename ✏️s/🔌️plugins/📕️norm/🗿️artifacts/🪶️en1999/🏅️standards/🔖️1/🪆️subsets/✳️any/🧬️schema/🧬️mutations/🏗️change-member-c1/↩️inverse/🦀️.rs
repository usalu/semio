//! ↩️ `change-member-c1` inverse: restores the previous `c1`.

use crate::change_member_c1::ChangeMemberC1;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberC1, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_c1::ChangeMemberC1> {
    Ok(match base.members.iter().find(|row| row.id == payload.member_id) {
        Some(row) => vec![Op::from(ChangeMemberC1 { member_id: payload.member_id.clone(), new_c1: row.c1 })],
        None => Vec::new(),
    })
}
