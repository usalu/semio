//! ↩️ `change-hollow-brace2-height` inverse: restores the previous `brace2_height`.

use crate::change_hollow_brace2_height::ChangeHollowBrace2Height;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeHollowBrace2Height, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_hollow_brace2_height::ChangeHollowBrace2Height> {
    Ok(match base.hollow_joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeHollowBrace2Height { id: payload.id.clone(), new_brace2_height: row.brace2_height })],
        None => Vec::new(),
    })
}
