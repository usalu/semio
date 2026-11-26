//! 🕳️ `remove-seismic` inverse — inserts the removed row back at its position; an absent row leaves nothing to restore.

use crate::remove_seismic::RemoveSeismic;
use crate::insert_seismic::InsertSeismic;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &RemoveSeismic, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_seismic::InsertSeismic> {
    Ok(match previous(payload, base) {
        Some(inverse) => vec![Op::from(inverse)],
        None => Vec::new(),
    })
}

fn previous(payload: &RemoveSeismic, base: &En1990Snapshot) -> Option<crate::insert_seismic::InsertSeismic> {
    base.seismics.get(payload.index).map(|item| InsertSeismic { index: Some(payload.index), item: item.clone() })
}
