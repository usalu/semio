//! ↩️ `change-hollow-brace2-angle` inverse: restores the previous `brace2_angle`.

use crate::change_hollow_brace2_angle::ChangeHollowBrace2Angle;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeHollowBrace2Angle, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_hollow_brace2_angle::ChangeHollowBrace2Angle> {
    Ok(match base.hollow_joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeHollowBrace2Angle { id: payload.id.clone(), new_brace2_angle: row.brace2_angle })],
        None => Vec::new(),
    })
}
