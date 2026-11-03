//! ↩️ `change-hollow-brace1-force` inverse: restores the previous `brace1_force`.

use crate::change_hollow_brace1_force::ChangeHollowBrace1Force;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeHollowBrace1Force, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_hollow_brace1_force::ChangeHollowBrace1Force> {
    Ok(match base.hollow_joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeHollowBrace1Force { id: payload.id.clone(), new_brace1_force: row.brace1_force })],
        None => Vec::new(),
    })
}
