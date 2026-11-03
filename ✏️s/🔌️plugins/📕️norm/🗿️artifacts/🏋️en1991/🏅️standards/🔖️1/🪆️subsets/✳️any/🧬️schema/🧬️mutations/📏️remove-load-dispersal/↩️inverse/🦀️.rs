//! ↩️ `remove-load-dispersal` inverse: inserts the removed row back at its position.

use crate::remove_load_dispersal::RemoveLoadDispersal;
use crate::insert_load_dispersal::InsertLoadDispersal;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &RemoveLoadDispersal, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_load_dispersal::InsertLoadDispersal> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveLoadDispersal, base: &En1991Snapshot) -> Option<InsertLoadDispersal> {
    let (index, row) = base.load_dispersals.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertLoadDispersal { index, item: row.clone() })
}
