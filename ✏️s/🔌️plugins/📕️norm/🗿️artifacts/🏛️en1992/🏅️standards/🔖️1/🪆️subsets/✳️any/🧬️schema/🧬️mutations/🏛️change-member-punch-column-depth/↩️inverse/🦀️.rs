//! ↩️ `change-member-punch-column-depth` inverse: restores the previous `punching.column_depth`.

use crate::change_member_punch_column_depth::ChangeMemberPunchColumnDepth;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberPunchColumnDepth, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_punch_column_depth::ChangeMemberPunchColumnDepth> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberPunchColumnDepth, base: &En1992Snapshot) -> Option<ChangeMemberPunchColumnDepth> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let object = r0.punching.as_ref()?;
    Some(ChangeMemberPunchColumnDepth { member_id: payload.member_id.clone(), new_punching_column_depth: object.column_depth })
}
