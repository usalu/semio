//! ↩️ `change-opening-width` inverse: restores the previous `width`.

use crate::change_opening_width::ChangeOpeningWidth;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeOpeningWidth, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_opening_width::ChangeOpeningWidth> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeOpeningWidth, base: &En1992Snapshot) -> Option<ChangeOpeningWidth> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let r1 = r0.openings.iter().find(|row| row.id == payload.opening_id)?;
    Some(ChangeOpeningWidth { member_id: payload.member_id.clone(), opening_id: payload.opening_id.clone(), new_width: r1.width })
}
