//! ↩️ `change-member-storey-height` inverse: restores the previous `storey_height`.

use crate::change_member_storey_height::ChangeMemberStoreyHeight;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberStoreyHeight, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_storey_height::ChangeMemberStoreyHeight> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberStoreyHeight, base: &En1990Snapshot) -> Option<ChangeMemberStoreyHeight> {
    let r0 = base.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberStoreyHeight { id: payload.id.clone(), new_storey_height: r0.storey_height })
}
