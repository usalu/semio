//! ↩️ `change-built-up-chord-inertia` inverse: restores the previous `chord_inertia`.

use crate::change_built_up_chord_inertia::ChangeBuiltUpChordInertia;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBuiltUpChordInertia, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_built_up_chord_inertia::ChangeBuiltUpChordInertia> {
    Ok(match base.built_up_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBuiltUpChordInertia { id: payload.id.clone(), new_chord_inertia: row.chord_inertia })],
        None => Vec::new(),
    })
}
