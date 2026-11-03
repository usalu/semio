//! ↩️ `change-wall-reinforcement-width` inverse: restores the previous `reinforcement_width`.

use crate::change_wall_reinforcement_width::ChangeWallReinforcementWidth;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeWallReinforcementWidth, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wall_reinforcement_width::ChangeWallReinforcementWidth> {
    Ok(match base.retaining_walls.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeWallReinforcementWidth { id: payload.id.clone(), new_reinforcement_width: row.reinforcement_width })],
        None => Vec::new(),
    })
}
