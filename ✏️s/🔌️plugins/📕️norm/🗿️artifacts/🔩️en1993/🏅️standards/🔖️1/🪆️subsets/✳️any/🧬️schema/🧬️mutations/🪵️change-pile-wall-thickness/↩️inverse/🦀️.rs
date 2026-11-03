//! ↩️ `change-pile-wall-thickness` inverse: restores the previous `wall_thickness`.

use crate::change_pile_wall_thickness::ChangePileWallThickness;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePileWallThickness, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_wall_thickness::ChangePileWallThickness> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileWallThickness { id: payload.id.clone(), new_wall_thickness: row.wall_thickness })],
        None => Vec::new(),
    })
}
