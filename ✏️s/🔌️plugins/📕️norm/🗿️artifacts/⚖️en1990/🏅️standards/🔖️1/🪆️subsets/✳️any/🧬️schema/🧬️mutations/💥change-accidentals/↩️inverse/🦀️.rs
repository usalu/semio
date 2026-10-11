//! 💥 `change-accidentals` inverse — removes the collection's new rows, then inserts the base rows in order; the rows are stored in reverse, as the store replays inverses reversed.

use super::ChangeAccidentals;
use crate::mutations::insert_accidental::InsertAccidental;
use crate::mutations::remove_accidental::RemoveAccidental;
use crate::{En1990Mutation, En1990Snapshot};

pub fn inverse(mutation: &ChangeAccidentals, base: &En1990Snapshot) -> Result<Vec<En1990Mutation>, semio_framework_value::ValueError> {
    let removed = mutation.new_accidentals.iter().map(|_| En1990Mutation::RemoveAccidental(RemoveAccidental { index: 0 }));
    let inserted = base.accidentals.iter().cloned().enumerate().map(|(index, item)| En1990Mutation::InsertAccidental(InsertAccidental { index: Some(index), item }));
    Ok(removed.chain(inserted).rev().collect())
}
