//! 💣 `insert-accidental` inverse — removes the row at the position the insert landed on.

use crate::insert_accidental::InsertAccidental;
use crate::remove_accidental::RemoveAccidental;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &InsertAccidental, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_accidental::RemoveAccidental> {
    Ok(match previous(payload, base) {
        Some(inverse) => vec![Op::from(inverse)],
        None => Vec::new(),
    })
}

fn previous(payload: &InsertAccidental, base: &En1990Snapshot) -> Option<crate::remove_accidental::RemoveAccidental> {
    Some(RemoveAccidental { index: payload.index.unwrap_or(usize::MAX).min(base.accidentals.len()) })
}
