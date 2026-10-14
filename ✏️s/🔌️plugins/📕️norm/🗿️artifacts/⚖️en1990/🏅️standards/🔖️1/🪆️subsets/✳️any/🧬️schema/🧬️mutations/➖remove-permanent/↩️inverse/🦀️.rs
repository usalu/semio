//! ➖ `remove-permanent` inverse — inserts the removed row back at its position; an absent row leaves nothing to restore.

use crate::remove_permanent::RemovePermanent;
use crate::insert_permanent::InsertPermanent;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &RemovePermanent, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_permanent::InsertPermanent> {
    Ok(base.permanents.get(payload.index).map(|item| vec![Op::from(InsertPermanent { index: Some(payload.index), item: item.clone() })]).unwrap_or_default())
}
