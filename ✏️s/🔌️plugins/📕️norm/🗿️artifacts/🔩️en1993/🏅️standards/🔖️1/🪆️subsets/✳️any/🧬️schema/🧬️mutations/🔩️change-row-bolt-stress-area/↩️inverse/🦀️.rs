//! ↩️ `change-row-bolt-stress-area` inverse: restores the previous `bolt_stress_area`.

use crate::change_row_bolt_stress_area::ChangeRowBoltStressArea;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeRowBoltStressArea, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_row_bolt_stress_area::ChangeRowBoltStressArea> {
    Ok(match base.joint_rows.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeRowBoltStressArea { id: payload.id.clone(), new_bolt_stress_area: row.bolt_stress_area })],
        None => Vec::new(),
    })
}
