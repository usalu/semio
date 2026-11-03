//! ↩️ `change-hollow-chord-fy` inverse: restores the previous `chord_fy`.

use crate::change_hollow_chord_fy::ChangeHollowChordFy;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeHollowChordFy, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_hollow_chord_fy::ChangeHollowChordFy> {
    Ok(match base.hollow_joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeHollowChordFy { id: payload.id.clone(), new_chord_fy: row.chord_fy })],
        None => Vec::new(),
    })
}
