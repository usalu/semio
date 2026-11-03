//! ↩️ `change-hollow-chord-width` inverse: restores the previous `chord_width`.

use crate::change_hollow_chord_width::ChangeHollowChordWidth;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeHollowChordWidth, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_hollow_chord_width::ChangeHollowChordWidth> {
    Ok(match base.hollow_joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeHollowChordWidth { id: payload.id.clone(), new_chord_width: row.chord_width })],
        None => Vec::new(),
    })
}
