//! ↩️ `change-fatigue-method` inverse: restores the previous `method`.

use crate::change_fatigue_method::ChangeFatigueMethod;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFatigueMethod, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fatigue_method::ChangeFatigueMethod> {
    Ok(match base.fatigue_details.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFatigueMethod { id: payload.id.clone(), new_method: row.method })],
        None => Vec::new(),
    })
}
