//! ↩️ `change-wall-wall-movement` inverse: restores the previous `wall_movement`.

use crate::change_wall_wall_movement::ChangeWallWallMovement;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeWallWallMovement, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wall_wall_movement::ChangeWallWallMovement> {
    Ok(match base.retaining_walls.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeWallWallMovement { id: payload.id.clone(), new_wall_movement: row.wall_movement })],
        None => Vec::new(),
    })
}
