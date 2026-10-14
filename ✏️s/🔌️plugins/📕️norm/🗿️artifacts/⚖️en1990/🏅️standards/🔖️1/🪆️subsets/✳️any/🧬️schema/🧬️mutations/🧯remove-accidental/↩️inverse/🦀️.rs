//! 🧯 `remove-accidental` inverse — inserts the removed row back at its position; an absent row leaves nothing to restore.

use crate::remove_accidental::RemoveAccidental;
use crate::insert_accidental::InsertAccidental;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &RemoveAccidental, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_accidental::InsertAccidental> {
    Ok(base.accidentals.get(payload.index).map(|item| vec![Op::from(InsertAccidental { index: Some(payload.index), item: item.clone() })]).unwrap_or_default())
}
