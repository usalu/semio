//! ↩️ `change-member-frame-height` inverse: restores the previous `frame_height`.

use crate::change_member_frame_height::ChangeMemberFrameHeight;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberFrameHeight, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_frame_height::ChangeMemberFrameHeight> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberFrameHeight, base: &En1990Snapshot) -> Option<ChangeMemberFrameHeight> {
    let r0 = base.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberFrameHeight { id: payload.id.clone(), new_frame_height: r0.frame_height })
}
