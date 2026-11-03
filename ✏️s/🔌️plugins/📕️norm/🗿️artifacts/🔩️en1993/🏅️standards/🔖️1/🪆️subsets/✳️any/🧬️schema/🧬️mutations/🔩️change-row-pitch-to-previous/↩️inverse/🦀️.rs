//! ↩️ `change-row-pitch-to-previous` inverse: restores the previous `pitch_to_previous`.

use crate::change_row_pitch_to_previous::ChangeRowPitchToPrevious;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeRowPitchToPrevious, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_row_pitch_to_previous::ChangeRowPitchToPrevious> {
    Ok(match base.joint_rows.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeRowPitchToPrevious { id: payload.id.clone(), new_pitch_to_previous: row.pitch_to_previous })],
        None => Vec::new(),
    })
}
