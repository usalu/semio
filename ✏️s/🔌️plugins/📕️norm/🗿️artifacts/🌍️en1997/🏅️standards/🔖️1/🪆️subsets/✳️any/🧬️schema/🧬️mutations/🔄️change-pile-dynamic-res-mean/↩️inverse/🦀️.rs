//! ↩️ `change-pile-dynamic-res-mean` inverse: restores the previous `dynamic_resistance_mean`.

use crate::change_pile_dynamic_res_mean::ChangePileDynamicResMean;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangePileDynamicResMean, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_dynamic_res_mean::ChangePileDynamicResMean> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileDynamicResMean { id: payload.id.clone(), new_dynamic_resistance_mean: row.dynamic_resistance_mean })],
        None => Vec::new(),
    })
}
