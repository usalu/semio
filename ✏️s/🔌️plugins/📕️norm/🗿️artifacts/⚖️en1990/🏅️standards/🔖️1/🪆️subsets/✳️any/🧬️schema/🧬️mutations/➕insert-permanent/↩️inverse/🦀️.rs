//! ➕ `insert-permanent` inverse — removes the row at the position the insert landed on.

use crate::insert_permanent::InsertPermanent;
use crate::remove_permanent::RemovePermanent;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &InsertPermanent, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_permanent::RemovePermanent> {
    Ok(vec![Op::from(RemovePermanent { index: payload.index.unwrap_or(usize::MAX).min(base.permanents.len()) })])
}
