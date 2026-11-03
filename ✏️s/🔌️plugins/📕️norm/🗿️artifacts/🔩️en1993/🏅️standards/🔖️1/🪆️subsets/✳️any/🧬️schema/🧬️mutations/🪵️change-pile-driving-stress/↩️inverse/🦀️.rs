//! ↩️ `change-pile-driving-stress` inverse: restores the previous `driving_stress`.

use crate::change_pile_driving_stress::ChangePileDrivingStress;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePileDrivingStress, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_driving_stress::ChangePileDrivingStress> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileDrivingStress { id: payload.id.clone(), new_driving_stress: row.driving_stress })],
        None => Vec::new(),
    })
}
