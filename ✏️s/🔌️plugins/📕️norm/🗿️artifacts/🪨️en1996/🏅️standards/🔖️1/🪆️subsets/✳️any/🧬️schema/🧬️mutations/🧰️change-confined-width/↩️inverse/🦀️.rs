//! ↩️ `change-confined-width` inverse: restores the previous `width_m`.

use crate::change_confined_width::ChangeConfinedWidth;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeConfinedWidth, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_confined_width::ChangeConfinedWidth> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeConfinedWidth, base: &En1996Snapshot) -> Option<ChangeConfinedWidth> {
    let r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    let r1 = r0.confined.iter().find(|row| row.id == payload.id)?;
    Some(ChangeConfinedWidth { wall_id: payload.wall_id.clone(), id: payload.id.clone(), new_width_m: r1.width_m })
}
