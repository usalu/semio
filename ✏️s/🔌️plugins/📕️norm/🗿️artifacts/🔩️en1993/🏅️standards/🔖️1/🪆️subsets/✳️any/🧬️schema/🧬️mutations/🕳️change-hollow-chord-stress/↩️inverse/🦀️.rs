//! ↩️ `change-hollow-chord-stress` inverse: restores the previous `chord_stress`.

use crate::change_hollow_chord_stress::ChangeHollowChordStress;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeHollowChordStress, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_hollow_chord_stress::ChangeHollowChordStress> {
    Ok(match base.hollow_joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeHollowChordStress { id: payload.id.clone(), new_chord_stress: row.chord_stress })],
        None => Vec::new(),
    })
}
