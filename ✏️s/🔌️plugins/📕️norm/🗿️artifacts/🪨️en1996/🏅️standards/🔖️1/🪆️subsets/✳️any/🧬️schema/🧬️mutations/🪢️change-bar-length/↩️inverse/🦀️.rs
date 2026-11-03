//! ↩️ `change-bar-length` inverse: restores the previous `length_m`.

use crate::change_bar_length::ChangeBarLength;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeBarLength, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bar_length::ChangeBarLength> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBarLength, base: &En1996Snapshot) -> Option<ChangeBarLength> {
    let r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    let r1 = r0.bars.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBarLength { wall_id: payload.wall_id.clone(), id: payload.id.clone(), new_length_m: r1.length_m })
}
