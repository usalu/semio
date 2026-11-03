//! ↩️ `change-pile-settling-thickness` inverse: restores the previous `settling_thickness`.

use crate::change_pile_settling_thickness::ChangePileSettlingThickness;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangePileSettlingThickness, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_settling_thickness::ChangePileSettlingThickness> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileSettlingThickness { id: payload.id.clone(), new_settling_thickness: row.settling_thickness })],
        None => Vec::new(),
    })
}
