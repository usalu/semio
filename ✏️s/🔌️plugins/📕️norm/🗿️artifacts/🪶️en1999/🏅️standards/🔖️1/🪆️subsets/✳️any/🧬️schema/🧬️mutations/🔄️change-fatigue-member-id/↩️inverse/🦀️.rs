//! ↩️ `change-fatigue-member-id` inverse: restores the previous `member_id`.

use crate::change_fatigue_member_id::ChangeFatigueMemberId;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeFatigueMemberId, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fatigue_member_id::ChangeFatigueMemberId> {
    Ok(match base.fatigue_details.iter().find(|row| row.id == payload.fatigue_detail_id) {
        Some(row) => vec![Op::from(ChangeFatigueMemberId { fatigue_detail_id: payload.fatigue_detail_id.clone(), new_member_id: row.member_id.clone() })],
        None => Vec::new(),
    })
}
