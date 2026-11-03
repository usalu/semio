//! ↩️ `change-hollow-brace1-width` inverse: restores the previous `brace1_width`.

use crate::change_hollow_brace1_width::ChangeHollowBrace1Width;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeHollowBrace1Width, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_hollow_brace1_width::ChangeHollowBrace1Width> {
    Ok(match base.hollow_joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeHollowBrace1Width { id: payload.id.clone(), new_brace1_width: row.brace1_width })],
        None => Vec::new(),
    })
}
