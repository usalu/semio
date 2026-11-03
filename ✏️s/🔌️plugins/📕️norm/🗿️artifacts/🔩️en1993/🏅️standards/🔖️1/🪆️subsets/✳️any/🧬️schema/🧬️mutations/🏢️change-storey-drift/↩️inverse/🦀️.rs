//! ↩️ `change-storey-drift` inverse: restores the previous `drift`.

use crate::change_storey_drift::ChangeStoreyDrift;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeStoreyDrift, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_storey_drift::ChangeStoreyDrift> {
    Ok(match base.storeys.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeStoreyDrift { id: payload.id.clone(), new_drift: row.drift })],
        None => Vec::new(),
    })
}
