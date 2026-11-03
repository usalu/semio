//! ↩️ `change-hollow-chord-height` inverse: restores the previous `chord_height`.

use crate::change_hollow_chord_height::ChangeHollowChordHeight;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeHollowChordHeight, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_hollow_chord_height::ChangeHollowChordHeight> {
    Ok(match base.hollow_joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeHollowChordHeight { id: payload.id.clone(), new_chord_height: row.chord_height })],
        None => Vec::new(),
    })
}
