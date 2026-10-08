//! 🌋️ `change-seismics` inverse — removes the collection's new rows back to front, then inserts the base rows in order; the rows are stored in reverse, as the store replays inverses reversed.

use super::ChangeSeismics;
use crate::mutations::insert_seismic::InsertSeismic;
use crate::mutations::remove_seismic::RemoveSeismic;
use crate::{En1990Mutation, En1990Snapshot};

pub fn inverse(mutation: &ChangeSeismics, base: &En1990Snapshot) -> Result<Vec<En1990Mutation>, semio_framework_value::ValueError> {
    let removed = (0..mutation.new_seismics.len()).rev().map(|index| En1990Mutation::RemoveSeismic(RemoveSeismic { index }));
    let inserted = base.seismics.iter().cloned().enumerate().map(|(index, item)| En1990Mutation::InsertSeismic(InsertSeismic { index, item }));
    Ok(removed.chain(inserted).rev().collect())
}
