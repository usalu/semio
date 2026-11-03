//! ↩️ `change-row-bolt-class` inverse: restores the previous `bolt_class`.

use crate::change_row_bolt_class::ChangeRowBoltClass;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeRowBoltClass, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_row_bolt_class::ChangeRowBoltClass> {
    Ok(match base.joint_rows.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeRowBoltClass { id: payload.id.clone(), new_bolt_class: row.bolt_class.clone() })],
        None => Vec::new(),
    })
}
