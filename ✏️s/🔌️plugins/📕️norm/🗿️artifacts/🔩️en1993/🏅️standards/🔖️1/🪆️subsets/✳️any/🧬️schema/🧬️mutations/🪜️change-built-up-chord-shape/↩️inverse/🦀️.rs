//! ↩️ `change-built-up-chord-shape` inverse: restores the previous `chord_shape`.

use crate::change_built_up_chord_shape::ChangeBuiltUpChordShape;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBuiltUpChordShape, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_built_up_chord_shape::ChangeBuiltUpChordShape> {
    Ok(match base.built_up_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBuiltUpChordShape { id: payload.id.clone(), new_chord_shape: row.chord_shape })],
        None => Vec::new(),
    })
}
