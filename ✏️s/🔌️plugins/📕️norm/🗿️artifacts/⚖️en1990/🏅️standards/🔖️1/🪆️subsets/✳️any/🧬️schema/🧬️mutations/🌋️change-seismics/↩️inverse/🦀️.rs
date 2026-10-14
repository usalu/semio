//! 🌋️ `change-seismics` inverse — removes the collection's new rows, then inserts the base rows in order; the rows are stored in reverse, as the store replays inverses reversed.

use crate::change_seismics::ChangeSeismics;
use crate::insert_seismic::InsertSeismic;
use crate::remove_seismic::RemoveSeismic;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(mutation: &ChangeSeismics, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_seismic::RemoveSeismic> + From<crate::insert_seismic::InsertSeismic> {
    let removed = mutation.new_seismics.iter().map(|_| Op::from(RemoveSeismic { index: 0 }));
    let inserted = base.seismics.iter().cloned().enumerate().map(|(index, item)| Op::from(InsertSeismic { index: Some(index), item }));
    Ok(removed.chain(inserted).rev().collect())
}
