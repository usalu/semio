//! ↩️ `change-tower-diameter` inverse: restores the previous `diameter`.

use crate::change_tower_diameter::ChangeTowerDiameter;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTowerDiameter, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tower_diameter::ChangeTowerDiameter> {
    Ok(match base.tower_legs.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTowerDiameter { id: payload.id.clone(), new_diameter: row.diameter })],
        None => Vec::new(),
    })
}
