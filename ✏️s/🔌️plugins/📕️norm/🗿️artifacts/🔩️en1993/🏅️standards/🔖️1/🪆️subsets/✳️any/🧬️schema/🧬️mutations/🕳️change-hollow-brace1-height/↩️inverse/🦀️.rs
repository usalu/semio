//! ↩️ `change-hollow-brace1-height` inverse: restores the previous `brace1_height`.

use crate::change_hollow_brace1_height::ChangeHollowBrace1Height;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeHollowBrace1Height, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_hollow_brace1_height::ChangeHollowBrace1Height> {
    Ok(match base.hollow_joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeHollowBrace1Height { id: payload.id.clone(), new_brace1_height: row.brace1_height })],
        None => Vec::new(),
    })
}
