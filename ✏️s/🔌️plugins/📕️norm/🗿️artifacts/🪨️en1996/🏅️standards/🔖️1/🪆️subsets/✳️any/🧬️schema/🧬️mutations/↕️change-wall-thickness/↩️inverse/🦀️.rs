//! ↩️ `change-wall-thickness` inverse: restores the previous `thickness_m`.

use crate::change_wall_thickness::ChangeWallThickness;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeWallThickness, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wall_thickness::ChangeWallThickness> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeWallThickness, base: &En1996Snapshot) -> Option<ChangeWallThickness> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeWallThickness { id: payload.id.clone(), new_thickness_m: r0.thickness_m })
}
