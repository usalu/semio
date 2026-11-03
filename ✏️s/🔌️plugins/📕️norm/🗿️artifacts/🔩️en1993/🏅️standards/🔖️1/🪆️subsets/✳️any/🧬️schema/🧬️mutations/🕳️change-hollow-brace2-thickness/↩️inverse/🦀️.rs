//! ↩️ `change-hollow-brace2-thickness` inverse: restores the previous `brace2_thickness`.

use crate::change_hollow_brace2_thickness::ChangeHollowBrace2Thickness;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeHollowBrace2Thickness, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_hollow_brace2_thickness::ChangeHollowBrace2Thickness> {
    Ok(match base.hollow_joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeHollowBrace2Thickness { id: payload.id.clone(), new_brace2_thickness: row.brace2_thickness })],
        None => Vec::new(),
    })
}
