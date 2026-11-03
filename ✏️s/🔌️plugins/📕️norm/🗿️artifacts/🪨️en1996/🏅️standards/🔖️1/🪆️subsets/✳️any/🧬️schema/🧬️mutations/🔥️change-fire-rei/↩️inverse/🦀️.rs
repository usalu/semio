//! ↩️ `change-fire-rei` inverse: restores the previous `fire_rei_min`.

use crate::change_fire_rei::ChangeFireRei;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeFireRei, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fire_rei::ChangeFireRei> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFireRei, base: &En1996Snapshot) -> Option<ChangeFireRei> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFireRei { id: payload.id.clone(), new_fire_rei_min: r0.fire_rei_min })
}
