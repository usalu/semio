//! ↩️ `change-chase-y` inverse: restores the previous `y_m`.

use crate::change_chase_y::ChangeChaseY;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeChaseY, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_chase_y::ChangeChaseY> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeChaseY, base: &En1996Snapshot) -> Option<ChangeChaseY> {
    let r0 = base.chases.iter().find(|row| row.id == payload.id)?;
    Some(ChangeChaseY { id: payload.id.clone(), new_y_m: r0.y_m })
}
