//! ↩️ `change-member-frame-drift` inverse: restores the previous `frame_drift`.

use crate::change_member_frame_drift::ChangeMemberFrameDrift;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberFrameDrift, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_frame_drift::ChangeMemberFrameDrift> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberFrameDrift, base: &En1990Snapshot) -> Option<ChangeMemberFrameDrift> {
    let r0 = base.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberFrameDrift { id: payload.id.clone(), new_frame_drift: r0.frame_drift })
}
