//! 🌋️ `change-seismics` inverse — removes the collection's new rows, then inserts the base rows in order; the rows are stored in reverse, as the store replays inverses reversed.

use super::ChangeSeismics;
use crate::mutations::insert_seismic::InsertSeismic;
use crate::mutations::remove_seismic::RemoveSeismic;
use crate::{En1990Mutation, En1990Snapshot};

pub fn inverse(mutation: &ChangeSeismics, base: &En1990Snapshot) -> Result<Vec<En1990Mutation>, semio_framework_value::ValueError> {
    let removed = mutation.new_seismics.iter().map(|_| En1990Mutation::RemoveSeismic(RemoveSeismic { index: 0 }));
    let inserted = base.seismics.iter().cloned().enumerate().map(|(index, item)| En1990Mutation::InsertSeismic(InsertSeismic { index: Some(index), item }));
    Ok(removed.chain(inserted).rev().collect())
}
