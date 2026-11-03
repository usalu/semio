//! ↩️ `change-hollow-gap` inverse: restores the previous `gap`.

use crate::change_hollow_gap::ChangeHollowGap;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeHollowGap, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_hollow_gap::ChangeHollowGap> {
    Ok(match base.hollow_joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeHollowGap { id: payload.id.clone(), new_gap: row.gap })],
        None => Vec::new(),
    })
}
