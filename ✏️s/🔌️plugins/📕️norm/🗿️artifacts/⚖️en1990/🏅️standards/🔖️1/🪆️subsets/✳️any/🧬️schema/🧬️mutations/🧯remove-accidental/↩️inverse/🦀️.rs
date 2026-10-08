//! 🧯 `remove-accidental` inverse — inserts the removed row back at its position; an absent row leaves nothing to restore.

use super::RemoveAccidental;
use crate::mutations::insert_accidental::InsertAccidental;
use crate::{En1990Mutation, En1990Snapshot};

pub fn inverse(payload: &RemoveAccidental, base: &En1990Snapshot) -> Result<Vec<En1990Mutation>, semio_framework_value::ValueError> {
    Ok(base.accidentals.get(payload.index).map(|item| vec![En1990Mutation::InsertAccidental(InsertAccidental { index: Some(payload.index), item: item.clone() })]).unwrap_or_default())
}
