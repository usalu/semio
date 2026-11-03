//! ↩️ `change-tower-bending-stiffness` inverse: restores the previous `bending_stiffness`.

use crate::change_tower_bending_stiffness::ChangeTowerBendingStiffness;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTowerBendingStiffness, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tower_bending_stiffness::ChangeTowerBendingStiffness> {
    Ok(match base.tower_legs.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTowerBendingStiffness { id: payload.id.clone(), new_bending_stiffness: row.bending_stiffness })],
        None => Vec::new(),
    })
}
