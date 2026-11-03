//! ↩️ `change-joint-bolts-per-row` inverse: restores the previous `bolts_per_row`.

use crate::change_joint_bolts_per_row::ChangeJointBoltsPerRow;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointBoltsPerRow, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_bolts_per_row::ChangeJointBoltsPerRow> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointBoltsPerRow { id: payload.id.clone(), new_bolts_per_row: row.bolts_per_row })],
        None => Vec::new(),
    })
}
