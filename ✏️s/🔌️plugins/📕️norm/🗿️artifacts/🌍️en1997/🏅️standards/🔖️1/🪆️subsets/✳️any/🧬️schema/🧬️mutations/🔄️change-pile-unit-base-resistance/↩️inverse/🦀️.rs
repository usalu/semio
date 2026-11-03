//! ↩️ `change-pile-unit-base-resistance` inverse: restores the previous `unit_base_resistance`.

use crate::change_pile_unit_base_resistance::ChangePileUnitBaseResistance;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangePileUnitBaseResistance, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_unit_base_resistance::ChangePileUnitBaseResistance> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileUnitBaseResistance { id: payload.id.clone(), new_unit_base_resistance: row.unit_base_resistance })],
        None => Vec::new(),
    })
}
