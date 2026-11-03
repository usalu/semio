//! ↩️ `change-built-up-chord-depth` inverse: restores the previous `chord_depth`.

use crate::change_built_up_chord_depth::ChangeBuiltUpChordDepth;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBuiltUpChordDepth, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_built_up_chord_depth::ChangeBuiltUpChordDepth> {
    Ok(match base.built_up_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBuiltUpChordDepth { id: payload.id.clone(), new_chord_depth: row.chord_depth })],
        None => Vec::new(),
    })
}
