//! ↩️ `change-tie-level` inverse: restores the previous `level_m`.

use crate::change_tie_level::ChangeTieLevel;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeTieLevel, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tie_level::ChangeTieLevel> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTieLevel, base: &En1996Snapshot) -> Option<ChangeTieLevel> {
    let r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    let r1 = r0.ties.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTieLevel { wall_id: payload.wall_id.clone(), id: payload.id.clone(), new_level_m: r1.level_m })
}
