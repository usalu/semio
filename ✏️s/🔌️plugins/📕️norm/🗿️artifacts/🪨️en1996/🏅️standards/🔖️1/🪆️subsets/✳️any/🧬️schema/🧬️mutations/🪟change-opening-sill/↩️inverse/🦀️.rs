//! ↩️ `change-opening-sill` inverse: restores the previous `sill_height_m`.

use crate::change_opening_sill::ChangeOpeningSill;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeOpeningSill, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_opening_sill::ChangeOpeningSill> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeOpeningSill, base: &En1996Snapshot) -> Option<ChangeOpeningSill> {
    let r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    let r1 = r0.openings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeOpeningSill { wall_id: payload.wall_id.clone(), id: payload.id.clone(), new_sill_height_m: r1.sill_height_m })
}
