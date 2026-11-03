//! ↩️ `change-bar-diameter` inverse: restores the previous `diameter_m`.

use crate::change_bar_diameter::ChangeBarDiameter;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeBarDiameter, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bar_diameter::ChangeBarDiameter> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBarDiameter, base: &En1996Snapshot) -> Option<ChangeBarDiameter> {
    let r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    let r1 = r0.bars.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBarDiameter { wall_id: payload.wall_id.clone(), id: payload.id.clone(), new_diameter_m: r1.diameter_m })
}
