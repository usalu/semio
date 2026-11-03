//! ↩️ `change-pile-unit-shaft-resistance` inverse: restores the previous `unit_shaft_resistance`.

use crate::change_pile_unit_shaft_resistance::ChangePileUnitShaftResistance;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangePileUnitShaftResistance, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_unit_shaft_resistance::ChangePileUnitShaftResistance> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileUnitShaftResistance { id: payload.id.clone(), new_unit_shaft_resistance: row.unit_shaft_resistance })],
        None => Vec::new(),
    })
}
