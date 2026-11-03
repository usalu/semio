//! ↩️ `change-hollow-chord-thickness` inverse: restores the previous `chord_thickness`.

use crate::change_hollow_chord_thickness::ChangeHollowChordThickness;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeHollowChordThickness, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_hollow_chord_thickness::ChangeHollowChordThickness> {
    Ok(match base.hollow_joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeHollowChordThickness { id: payload.id.clone(), new_chord_thickness: row.chord_thickness })],
        None => Vec::new(),
    })
}
