//! ↩️ `change-member-punch-col-position` inverse: restores the previous `punching.column_position`.

use crate::change_member_punch_col_position::ChangeMemberPunchColPosition;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberPunchColPosition, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_punch_col_position::ChangeMemberPunchColPosition> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberPunchColPosition, base: &En1992Snapshot) -> Option<ChangeMemberPunchColPosition> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let object = r0.punching.as_ref()?;
    Some(ChangeMemberPunchColPosition { member_id: payload.member_id.clone(), new_punching_column_position: object.column_position })
}
