//! ↩️ `change-built-up-chord-area` inverse: restores the previous `chord_area`.

use crate::change_built_up_chord_area::ChangeBuiltUpChordArea;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBuiltUpChordArea, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_built_up_chord_area::ChangeBuiltUpChordArea> {
    Ok(match base.built_up_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBuiltUpChordArea { id: payload.id.clone(), new_chord_area: row.chord_area })],
        None => Vec::new(),
    })
}
