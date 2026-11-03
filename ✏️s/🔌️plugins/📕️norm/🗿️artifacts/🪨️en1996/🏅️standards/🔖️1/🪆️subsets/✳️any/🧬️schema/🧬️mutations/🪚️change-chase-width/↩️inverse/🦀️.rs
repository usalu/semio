//! ↩️ `change-chase-width` inverse: restores the previous `width_m`.

use crate::change_chase_width::ChangeChaseWidth;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeChaseWidth, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_chase_width::ChangeChaseWidth> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeChaseWidth, base: &En1996Snapshot) -> Option<ChangeChaseWidth> {
    let r0 = base.chases.iter().find(|row| row.id == payload.id)?;
    Some(ChangeChaseWidth { id: payload.id.clone(), new_width_m: r0.width_m })
}
