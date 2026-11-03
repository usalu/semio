//! ↩️ `change-fatigue-n-cycles` inverse: restores the previous `n_cycles`.

use crate::change_fatigue_n_cycles::ChangeFatigueNCycles;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeFatigueNCycles, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fatigue_n_cycles::ChangeFatigueNCycles> {
    Ok(match base.fatigue_details.iter().find(|row| row.id == payload.fatigue_detail_id) {
        Some(row) => vec![Op::from(ChangeFatigueNCycles { fatigue_detail_id: payload.fatigue_detail_id.clone(), new_n_cycles: row.n_cycles })],
        None => Vec::new(),
    })
}
