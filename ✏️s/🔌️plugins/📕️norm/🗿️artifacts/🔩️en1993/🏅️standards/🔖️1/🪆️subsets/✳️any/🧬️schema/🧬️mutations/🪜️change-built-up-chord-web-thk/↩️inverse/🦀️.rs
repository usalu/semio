//! ↩️ `change-built-up-chord-web-thk` inverse: restores the previous `chord_web_thickness`.

use crate::change_built_up_chord_web_thk::ChangeBuiltUpChordWebThk;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBuiltUpChordWebThk, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_built_up_chord_web_thk::ChangeBuiltUpChordWebThk> {
    Ok(match base.built_up_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBuiltUpChordWebThk { id: payload.id.clone(), new_chord_web_thickness: row.chord_web_thickness })],
        None => Vec::new(),
    })
}
