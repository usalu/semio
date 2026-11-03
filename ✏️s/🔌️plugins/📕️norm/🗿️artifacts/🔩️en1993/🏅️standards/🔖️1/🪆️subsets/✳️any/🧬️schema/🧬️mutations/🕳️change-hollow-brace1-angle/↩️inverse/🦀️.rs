//! ↩️ `change-hollow-brace1-angle` inverse: restores the previous `brace1_angle`.

use crate::change_hollow_brace1_angle::ChangeHollowBrace1Angle;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeHollowBrace1Angle, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_hollow_brace1_angle::ChangeHollowBrace1Angle> {
    Ok(match base.hollow_joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeHollowBrace1Angle { id: payload.id.clone(), new_brace1_angle: row.brace1_angle })],
        None => Vec::new(),
    })
}
