//! ↩️ `change-row-bolts` inverse: restores the previous `bolts`.

use crate::change_row_bolts::ChangeRowBolts;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeRowBolts, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_row_bolts::ChangeRowBolts> {
    Ok(match base.joint_rows.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeRowBolts { id: payload.id.clone(), new_bolts: row.bolts })],
        None => Vec::new(),
    })
}
