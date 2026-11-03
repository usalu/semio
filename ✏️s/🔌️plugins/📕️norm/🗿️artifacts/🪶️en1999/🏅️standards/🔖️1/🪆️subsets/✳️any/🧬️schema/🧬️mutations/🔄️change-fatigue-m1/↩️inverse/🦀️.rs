//! ↩️ `change-fatigue-m1` inverse: restores the previous `m1`.

use crate::change_fatigue_m1::ChangeFatigueM1;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeFatigueM1, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fatigue_m1::ChangeFatigueM1> {
    Ok(match base.fatigue_details.iter().find(|row| row.id == payload.fatigue_detail_id) {
        Some(row) => vec![Op::from(ChangeFatigueM1 { fatigue_detail_id: payload.fatigue_detail_id.clone(), new_m1: row.m1 })],
        None => Vec::new(),
    })
}
