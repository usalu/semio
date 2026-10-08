//! 💣 `insert-accidental` inverse — removes the row at the position the insert landed on.

use super::InsertAccidental;
use crate::mutations::remove_accidental::RemoveAccidental;
use crate::{En1990Mutation, En1990Snapshot};

pub fn inverse(payload: &InsertAccidental, base: &En1990Snapshot) -> Result<Vec<En1990Mutation>, semio_framework_value::ValueError> {
    Ok(vec![En1990Mutation::RemoveAccidental(RemoveAccidental { index: payload.index.min(base.accidentals.len()) })])
}
