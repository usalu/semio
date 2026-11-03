//! ↩️ `change-wall-rebar-friction` inverse: restores the previous `reinforcement_friction`.

use crate::change_wall_rebar_friction::ChangeWallRebarFriction;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeWallRebarFriction, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wall_rebar_friction::ChangeWallRebarFriction> {
    Ok(match base.retaining_walls.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeWallRebarFriction { id: payload.id.clone(), new_reinforcement_friction: row.reinforcement_friction })],
        None => Vec::new(),
    })
}
