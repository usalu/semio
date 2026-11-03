//! ↩️ `change-frame-bar-member-id` inverse: restores the previous `member_id`.

use crate::change_frame_bar_member_id::ChangeFrameBarMemberId;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFrameBarMemberId, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_frame_bar_member_id::ChangeFrameBarMemberId> {
    Ok(match base.frame_bars.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFrameBarMemberId { id: payload.id.clone(), new_member_id: row.member_id.clone() })],
        None => Vec::new(),
    })
}
