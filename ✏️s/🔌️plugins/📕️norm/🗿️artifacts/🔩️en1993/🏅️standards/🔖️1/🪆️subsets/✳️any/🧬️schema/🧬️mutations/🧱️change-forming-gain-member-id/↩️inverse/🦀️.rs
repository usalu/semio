//! ↩️ `change-forming-gain-member-id` inverse: restores the previous `member_id`.

use crate::change_forming_gain_member_id::ChangeFormingGainMemberId;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFormingGainMemberId, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_forming_gain_member_id::ChangeFormingGainMemberId> {
    Ok(match base.forming_gains.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFormingGainMemberId { id: payload.id.clone(), new_member_id: row.member_id.clone() })],
        None => Vec::new(),
    })
}
