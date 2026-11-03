//! ↩️ `change-tower-log-decrement` inverse: restores the previous `log_decrement`.

use crate::change_tower_log_decrement::ChangeTowerLogDecrement;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTowerLogDecrement, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tower_log_decrement::ChangeTowerLogDecrement> {
    Ok(match base.tower_legs.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTowerLogDecrement { id: payload.id.clone(), new_log_decrement: row.log_decrement })],
        None => Vec::new(),
    })
}
