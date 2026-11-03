//! ↩️ `change-flange-thickness` inverse: restores the previous `thickness_m`.

use crate::change_flange_thickness::ChangeFlangeThickness;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeFlangeThickness, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_flange_thickness::ChangeFlangeThickness> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFlangeThickness, base: &En1996Snapshot) -> Option<ChangeFlangeThickness> {
    let r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    let r1 = r0.flanges.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFlangeThickness { wall_id: payload.wall_id.clone(), id: payload.id.clone(), new_thickness_m: r1.thickness_m })
}
