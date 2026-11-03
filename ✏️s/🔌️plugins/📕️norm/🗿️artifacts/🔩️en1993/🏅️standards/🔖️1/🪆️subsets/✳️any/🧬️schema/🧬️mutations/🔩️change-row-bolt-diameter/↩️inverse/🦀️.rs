//! ↩️ `change-row-bolt-diameter` inverse: restores the previous `bolt_diameter`.

use crate::change_row_bolt_diameter::ChangeRowBoltDiameter;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeRowBoltDiameter, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_row_bolt_diameter::ChangeRowBoltDiameter> {
    Ok(match base.joint_rows.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeRowBoltDiameter { id: payload.id.clone(), new_bolt_diameter: row.bolt_diameter })],
        None => Vec::new(),
    })
}
