//! 🕳️ `remove-seismic` inverse — inserts the removed row back at its position; an absent row leaves nothing to restore.

use crate::remove_seismic::RemoveSeismic;
use crate::insert_seismic::InsertSeismic;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &RemoveSeismic, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_seismic::InsertSeismic> {
    Ok(base.seismics.get(payload.index).map(|item| vec![Op::from(InsertSeismic { index: Some(payload.index), item: item.clone() })]).unwrap_or_default())
}
