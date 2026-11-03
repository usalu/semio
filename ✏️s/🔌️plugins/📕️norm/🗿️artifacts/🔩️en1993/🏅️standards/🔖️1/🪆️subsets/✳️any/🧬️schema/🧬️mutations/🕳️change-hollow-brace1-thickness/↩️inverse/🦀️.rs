//! ↩️ `change-hollow-brace1-thickness` inverse: restores the previous `brace1_thickness`.

use crate::change_hollow_brace1_thickness::ChangeHollowBrace1Thickness;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeHollowBrace1Thickness, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_hollow_brace1_thickness::ChangeHollowBrace1Thickness> {
    Ok(match base.hollow_joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeHollowBrace1Thickness { id: payload.id.clone(), new_brace1_thickness: row.brace1_thickness })],
        None => Vec::new(),
    })
}
