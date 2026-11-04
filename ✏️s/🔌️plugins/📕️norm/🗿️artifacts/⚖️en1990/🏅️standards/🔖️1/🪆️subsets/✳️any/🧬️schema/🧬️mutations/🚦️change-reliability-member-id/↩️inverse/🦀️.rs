//! ↩️ `change-reliability-member-id` inverse: restores the previous `member_id`.

use crate::change_reliability_member_id::ChangeReliabilityMemberId;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeReliabilityMemberId, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_reliability_member_id::ChangeReliabilityMemberId> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeReliabilityMemberId, base: &En1990Snapshot) -> Option<ChangeReliabilityMemberId> {
    let r0 = base.reliability_analyses.iter().find(|row| row.id == payload.id)?;
    Some(ChangeReliabilityMemberId { id: payload.id.clone(), new_member_id: r0.member_id.clone() })
}
