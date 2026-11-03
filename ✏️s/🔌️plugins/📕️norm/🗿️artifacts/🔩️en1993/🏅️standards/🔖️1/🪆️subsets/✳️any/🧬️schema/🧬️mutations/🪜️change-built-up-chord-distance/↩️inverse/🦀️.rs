//! ↩️ `change-built-up-chord-distance` inverse: restores the previous `chord_distance`.

use crate::change_built_up_chord_distance::ChangeBuiltUpChordDistance;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBuiltUpChordDistance, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_built_up_chord_distance::ChangeBuiltUpChordDistance> {
    Ok(match base.built_up_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBuiltUpChordDistance { id: payload.id.clone(), new_chord_distance: row.chord_distance })],
        None => Vec::new(),
    })
}
