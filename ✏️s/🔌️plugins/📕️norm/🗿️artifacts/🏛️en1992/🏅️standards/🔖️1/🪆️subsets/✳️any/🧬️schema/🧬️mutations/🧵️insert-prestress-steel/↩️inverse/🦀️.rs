//! ↩️ `insert-prestress-steel` inverse: removes the inserted row again.

use crate::insert_prestress_steel::InsertPrestressSteel;
use crate::remove_prestress_steel::RemovePrestressSteel;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &InsertPrestressSteel, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_prestress_steel::RemovePrestressSteel> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertPrestressSteel, base: &En1992Snapshot) -> Option<RemovePrestressSteel> {
    let index = payload.index;
    Some(RemovePrestressSteel { index })
}
