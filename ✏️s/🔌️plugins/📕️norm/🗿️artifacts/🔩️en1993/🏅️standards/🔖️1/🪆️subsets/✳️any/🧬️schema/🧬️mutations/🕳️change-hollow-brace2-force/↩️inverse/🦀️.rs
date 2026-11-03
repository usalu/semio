//! ↩️ `change-hollow-brace2-force` inverse: restores the previous `brace2_force`.

use crate::change_hollow_brace2_force::ChangeHollowBrace2Force;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeHollowBrace2Force, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_hollow_brace2_force::ChangeHollowBrace2Force> {
    Ok(match base.hollow_joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeHollowBrace2Force { id: payload.id.clone(), new_brace2_force: row.brace2_force })],
        None => Vec::new(),
    })
}
