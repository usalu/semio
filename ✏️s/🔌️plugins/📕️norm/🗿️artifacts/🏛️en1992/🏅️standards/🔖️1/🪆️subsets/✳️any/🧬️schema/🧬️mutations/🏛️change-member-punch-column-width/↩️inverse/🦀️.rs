//! ↩️ `change-member-punch-column-width` inverse: restores the previous `punching.column_width`.

use crate::change_member_punch_column_width::ChangeMemberPunchColumnWidth;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberPunchColumnWidth, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_punch_column_width::ChangeMemberPunchColumnWidth> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberPunchColumnWidth, base: &En1992Snapshot) -> Option<ChangeMemberPunchColumnWidth> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let object = r0.punching.as_ref()?;
    Some(ChangeMemberPunchColumnWidth { member_id: payload.member_id.clone(), new_punching_column_width: object.column_width })
}
