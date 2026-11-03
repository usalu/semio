//! ↩️ `change-built-up-chord-flange-thickness` inverse: restores the previous `chord_flange_thickness`.

use crate::change_built_up_chord_flange_thickness::ChangeBuiltUpChordFlangeThickness;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBuiltUpChordFlangeThickness, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_built_up_chord_flange_thickness::ChangeBuiltUpChordFlangeThickness> {
    Ok(match base.built_up_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBuiltUpChordFlangeThickness { id: payload.id.clone(), new_chord_flange_thickness: row.chord_flange_thickness })],
        None => Vec::new(),
    })
}
