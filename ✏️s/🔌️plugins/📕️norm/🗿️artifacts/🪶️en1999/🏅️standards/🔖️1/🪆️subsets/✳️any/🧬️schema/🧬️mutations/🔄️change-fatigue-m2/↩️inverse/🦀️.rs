//! ↩️ `change-fatigue-m2` inverse: restores the previous `m2`.

use crate::change_fatigue_m2::ChangeFatigueM2;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeFatigueM2, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fatigue_m2::ChangeFatigueM2> {
    Ok(match base.fatigue_details.iter().find(|row| row.id == payload.fatigue_detail_id) {
        Some(row) => vec![Op::from(ChangeFatigueM2 { fatigue_detail_id: payload.fatigue_detail_id.clone(), new_m2: row.m2 })],
        None => Vec::new(),
    })
}
