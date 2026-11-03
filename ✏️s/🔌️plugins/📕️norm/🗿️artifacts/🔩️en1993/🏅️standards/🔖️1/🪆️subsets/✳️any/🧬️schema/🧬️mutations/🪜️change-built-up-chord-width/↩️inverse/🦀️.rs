//! ↩️ `change-built-up-chord-width` inverse: restores the previous `chord_width`.

use crate::change_built_up_chord_width::ChangeBuiltUpChordWidth;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBuiltUpChordWidth, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_built_up_chord_width::ChangeBuiltUpChordWidth> {
    Ok(match base.built_up_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBuiltUpChordWidth { id: payload.id.clone(), new_chord_width: row.chord_width })],
        None => Vec::new(),
    })
}
