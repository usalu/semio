//! ↩️ `change-wall-reinforcement-spacing` inverse: restores the previous `reinforcement_spacing`.

use crate::change_wall_reinforcement_spacing::ChangeWallReinforcementSpacing;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeWallReinforcementSpacing, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wall_reinforcement_spacing::ChangeWallReinforcementSpacing> {
    Ok(match base.retaining_walls.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeWallReinforcementSpacing { id: payload.id.clone(), new_reinforcement_spacing: row.reinforcement_spacing })],
        None => Vec::new(),
    })
}
