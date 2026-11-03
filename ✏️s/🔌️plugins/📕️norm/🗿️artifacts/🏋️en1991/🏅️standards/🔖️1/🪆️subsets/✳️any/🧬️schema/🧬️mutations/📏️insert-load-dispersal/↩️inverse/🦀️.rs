//! ↩️ `insert-load-dispersal` inverse: removes the inserted row again.

use crate::insert_load_dispersal::InsertLoadDispersal;
use crate::remove_load_dispersal::RemoveLoadDispersal;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &InsertLoadDispersal, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_load_dispersal::RemoveLoadDispersal> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertLoadDispersal, base: &En1991Snapshot) -> Option<RemoveLoadDispersal> {
    Some(RemoveLoadDispersal { id: payload.item.id.clone() })
}
