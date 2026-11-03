//! ↩️ `change-pile-dynamic-res-min` inverse: restores the previous `dynamic_resistance_min`.

use crate::change_pile_dynamic_res_min::ChangePileDynamicResMin;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangePileDynamicResMin, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_dynamic_res_min::ChangePileDynamicResMin> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileDynamicResMin { id: payload.id.clone(), new_dynamic_resistance_min: row.dynamic_resistance_min })],
        None => Vec::new(),
    })
}
