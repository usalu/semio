//! ↩️ `change-hollow-brace2-width` inverse: restores the previous `brace2_width`.

use crate::change_hollow_brace2_width::ChangeHollowBrace2Width;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeHollowBrace2Width, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_hollow_brace2_width::ChangeHollowBrace2Width> {
    Ok(match base.hollow_joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeHollowBrace2Width { id: payload.id.clone(), new_brace2_width: row.brace2_width })],
        None => Vec::new(),
    })
}
