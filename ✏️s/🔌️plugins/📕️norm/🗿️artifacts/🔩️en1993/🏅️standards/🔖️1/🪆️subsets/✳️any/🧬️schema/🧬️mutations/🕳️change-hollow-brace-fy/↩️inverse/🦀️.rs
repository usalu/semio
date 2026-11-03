//! ↩️ `change-hollow-brace-fy` inverse: restores the previous `brace_fy`.

use crate::change_hollow_brace_fy::ChangeHollowBraceFy;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeHollowBraceFy, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_hollow_brace_fy::ChangeHollowBraceFy> {
    Ok(match base.hollow_joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeHollowBraceFy { id: payload.id.clone(), new_brace_fy: row.brace_fy })],
        None => Vec::new(),
    })
}
