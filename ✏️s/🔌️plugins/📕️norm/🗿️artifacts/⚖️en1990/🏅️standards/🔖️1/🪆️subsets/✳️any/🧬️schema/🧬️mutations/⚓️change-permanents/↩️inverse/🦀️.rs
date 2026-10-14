//! ⚓️ `change-permanents` inverse — removes the collection's new rows, then inserts the base rows in order; the rows are stored in reverse, as the store replays inverses reversed.

use crate::change_permanents::ChangePermanents;
use crate::insert_permanent::InsertPermanent;
use crate::remove_permanent::RemovePermanent;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(mutation: &ChangePermanents, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_permanent::RemovePermanent> + From<crate::insert_permanent::InsertPermanent> {
    let removed = mutation.new_permanents.iter().map(|_| Op::from(RemovePermanent { index: 0 }));
    let inserted = base.permanents.iter().cloned().enumerate().map(|(index, item)| Op::from(InsertPermanent { index: Some(index), item }));
    Ok(removed.chain(inserted).rev().collect())
}
