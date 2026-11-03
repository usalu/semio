//! ↩️ `change-fire-member-id` inverse: restores the previous `member_id`.

use crate::change_fire_member_id::ChangeFireMemberId;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFireMemberId, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fire_member_id::ChangeFireMemberId> {
    Ok(match base.fire_exposures.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFireMemberId { id: payload.id.clone(), new_member_id: row.member_id.clone() })],
        None => Vec::new(),
    })
}
