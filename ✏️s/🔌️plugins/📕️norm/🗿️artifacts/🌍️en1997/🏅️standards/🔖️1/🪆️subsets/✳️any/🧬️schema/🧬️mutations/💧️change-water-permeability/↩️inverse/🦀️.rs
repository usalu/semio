//! ↩️ `change-water-permeability` inverse: restores the previous `permeability`.

use crate::change_water_permeability::ChangeWaterPermeability;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeWaterPermeability, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_water_permeability::ChangeWaterPermeability> {
    Ok(match base.layer_waters.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeWaterPermeability { id: payload.id.clone(), new_permeability: row.permeability })],
        None => Vec::new(),
    })
}
