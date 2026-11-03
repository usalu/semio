//! ↩️ `change-tributary-area` inverse: restores the previous `tributary_area_m2`.

use crate::change_tributary_area::ChangeTributaryArea;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeTributaryArea, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tributary_area::ChangeTributaryArea> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTributaryArea, base: &En1996Snapshot) -> Option<ChangeTributaryArea> {
    let r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    let r1 = r0.load_cases.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTributaryArea { wall_id: payload.wall_id.clone(), id: payload.id.clone(), new_tributary_area_m2: r1.tributary_area_m2 })
}
