//! ↩️ `change-bridge-member-id` inverse: restores the previous `member_id`.

use crate::change_bridge_member_id::ChangeBridgeMemberId;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeMemberId, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_member_id::ChangeBridgeMemberId> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBridgeMemberId, base: &En1990Snapshot) -> Option<ChangeBridgeMemberId> {
    let r0 = base.bridge_sls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBridgeMemberId { id: payload.id.clone(), new_member_id: r0.member_id.clone() })
}
