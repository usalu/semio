//! 💥 `change-accidentals` inverse — removes the collection's new rows, then inserts the base rows in order; the rows are stored in reverse, as the store replays inverses reversed.

use crate::change_accidentals::ChangeAccidentals;
use crate::insert_accidental::InsertAccidental;
use crate::remove_accidental::RemoveAccidental;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(mutation: &ChangeAccidentals, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_accidental::RemoveAccidental> + From<crate::insert_accidental::InsertAccidental> {
    let removed = mutation.new_accidentals.iter().map(|_| Op::from(RemoveAccidental { index: 0 }));
    let inserted = base.accidentals.iter().cloned().enumerate().map(|(index, item)| Op::from(InsertAccidental { index: Some(index), item }));
    Ok(removed.chain(inserted).rev().collect())
}
