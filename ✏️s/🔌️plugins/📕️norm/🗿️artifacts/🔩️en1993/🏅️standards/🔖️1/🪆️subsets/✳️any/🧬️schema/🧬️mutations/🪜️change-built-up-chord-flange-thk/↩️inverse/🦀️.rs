//! ↩️ `change-built-up-chord-flange-thk` inverse: restores the previous `chord_flange_thickness`.

use crate::change_built_up_chord_flange_thk::ChangeBuiltUpChordFlangeThk;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBuiltUpChordFlangeThk, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_built_up_chord_flange_thk::ChangeBuiltUpChordFlangeThk> {
    Ok(match base.built_up_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBuiltUpChordFlangeThk { id: payload.id.clone(), new_chord_flange_thickness: row.chord_flange_thickness })],
        None => Vec::new(),
    })
}
