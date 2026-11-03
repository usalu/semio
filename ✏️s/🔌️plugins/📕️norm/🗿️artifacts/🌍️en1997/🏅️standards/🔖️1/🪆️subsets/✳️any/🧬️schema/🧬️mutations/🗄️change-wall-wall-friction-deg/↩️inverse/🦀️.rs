//! ↩️ `change-wall-wall-friction-deg` inverse: restores the previous `wall_friction_deg`.

use crate::change_wall_wall_friction_deg::ChangeWallWallFrictionDeg;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeWallWallFrictionDeg, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wall_wall_friction_deg::ChangeWallWallFrictionDeg> {
    Ok(match base.retaining_walls.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeWallWallFrictionDeg { id: payload.id.clone(), new_wall_friction_deg: row.wall_friction_deg })],
        None => Vec::new(),
    })
}
