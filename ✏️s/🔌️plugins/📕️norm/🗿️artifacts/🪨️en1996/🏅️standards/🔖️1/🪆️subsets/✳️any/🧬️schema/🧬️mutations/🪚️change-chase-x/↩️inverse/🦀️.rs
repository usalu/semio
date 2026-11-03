//! ↩️ `change-chase-x` inverse: restores the previous `x_m`.

use crate::change_chase_x::ChangeChaseX;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeChaseX, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_chase_x::ChangeChaseX> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeChaseX, base: &En1996Snapshot) -> Option<ChangeChaseX> {
    let r0 = base.chases.iter().find(|row| row.id == payload.id)?;
    Some(ChangeChaseX { id: payload.id.clone(), new_x_m: r0.x_m })
}
