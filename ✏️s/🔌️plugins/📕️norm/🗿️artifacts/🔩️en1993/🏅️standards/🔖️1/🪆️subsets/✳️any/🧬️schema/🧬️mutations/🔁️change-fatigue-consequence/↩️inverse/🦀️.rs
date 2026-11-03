//! ↩️ `change-fatigue-consequence` inverse: restores the previous `consequence`.

use crate::change_fatigue_consequence::ChangeFatigueConsequence;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFatigueConsequence, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fatigue_consequence::ChangeFatigueConsequence> {
    Ok(match base.fatigue_details.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFatigueConsequence { id: payload.id.clone(), new_consequence: row.consequence })],
        None => Vec::new(),
    })
}
