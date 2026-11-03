//! ↩️ `change-opening-y` inverse: restores the previous `y`.

use crate::change_opening_y::ChangeOpeningY;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeOpeningY, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_opening_y::ChangeOpeningY> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeOpeningY, base: &En1992Snapshot) -> Option<ChangeOpeningY> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let r1 = r0.openings.iter().find(|row| row.id == payload.opening_id)?;
    Some(ChangeOpeningY { member_id: payload.member_id.clone(), opening_id: payload.opening_id.clone(), new_y: r1.y })
}
