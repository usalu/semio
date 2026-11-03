//! ↩️ `change-confined-steel` inverse: restores the previous `as_m2`.

use crate::change_confined_steel::ChangeConfinedSteel;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeConfinedSteel, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_confined_steel::ChangeConfinedSteel> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeConfinedSteel, base: &En1996Snapshot) -> Option<ChangeConfinedSteel> {
    let r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    let r1 = r0.confined.iter().find(|row| row.id == payload.id)?;
    Some(ChangeConfinedSteel { wall_id: payload.wall_id.clone(), id: payload.id.clone(), new_as_m2: r1.as_m2 })
}
