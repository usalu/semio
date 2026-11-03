//! ↩️ `change-row-bolt-to-web` inverse: restores the previous `bolt_to_web`.

use crate::change_row_bolt_to_web::ChangeRowBoltToWeb;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeRowBoltToWeb, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_row_bolt_to_web::ChangeRowBoltToWeb> {
    Ok(match base.joint_rows.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeRowBoltToWeb { id: payload.id.clone(), new_bolt_to_web: row.bolt_to_web })],
        None => Vec::new(),
    })
}
