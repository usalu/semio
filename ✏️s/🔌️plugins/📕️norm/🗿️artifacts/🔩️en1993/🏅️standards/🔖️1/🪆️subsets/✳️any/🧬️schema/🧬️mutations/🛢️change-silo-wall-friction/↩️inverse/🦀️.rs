//! ↩️ `change-silo-wall-friction` inverse: restores the previous `wall_friction`.

use crate::change_silo_wall_friction::ChangeSiloWallFriction;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSiloWallFriction, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_wall_friction::ChangeSiloWallFriction> {
    Ok(match base.silo_shells.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSiloWallFriction { id: payload.id.clone(), new_wall_friction: row.wall_friction })],
        None => Vec::new(),
    })
}
