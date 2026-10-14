//! 🌋 `insert-seismic` inverse — removes the row at the position the insert landed on.

use crate::insert_seismic::InsertSeismic;
use crate::remove_seismic::RemoveSeismic;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &InsertSeismic, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_seismic::RemoveSeismic> {
    Ok(vec![Op::from(RemoveSeismic { index: payload.index.unwrap_or(usize::MAX).min(base.seismics.len()) })])
}
