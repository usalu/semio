//! ↩️ `change-critical-region-member-id` inverse: restores the previous `member_id`.

use crate::change_critical_region_member_id::ChangeCriticalRegionMemberId;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeCriticalRegionMemberId, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_critical_region_member_id::ChangeCriticalRegionMemberId> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeCriticalRegionMemberId, base: &En1998Snapshot) -> Option<ChangeCriticalRegionMemberId> {
    let r0 = base.critical_regions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeCriticalRegionMemberId { id: payload.id.clone(), new_member_id: r0.member_id.clone() })
}
