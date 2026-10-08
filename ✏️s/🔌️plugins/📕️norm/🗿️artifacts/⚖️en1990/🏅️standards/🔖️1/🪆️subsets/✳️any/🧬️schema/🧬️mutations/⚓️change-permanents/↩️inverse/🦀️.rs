//! ⚓️ `change-permanents` inverse — removes the collection's new rows, then inserts the base rows in order; the rows are stored in reverse, as the store replays inverses reversed.

use super::ChangePermanents;
use crate::mutations::insert_permanent::InsertPermanent;
use crate::mutations::remove_permanent::RemovePermanent;
use crate::{En1990Mutation, En1990Snapshot};

pub fn inverse(mutation: &ChangePermanents, base: &En1990Snapshot) -> Result<Vec<En1990Mutation>, semio_framework_value::ValueError> {
    let removed = mutation.new_permanents.iter().map(|_| En1990Mutation::RemovePermanent(RemovePermanent { index: 0 }));
    let inserted = base.permanents.iter().cloned().enumerate().map(|(index, item)| En1990Mutation::InsertPermanent(InsertPermanent { index, item }));
    Ok(removed.chain(inserted).rev().collect())
}
