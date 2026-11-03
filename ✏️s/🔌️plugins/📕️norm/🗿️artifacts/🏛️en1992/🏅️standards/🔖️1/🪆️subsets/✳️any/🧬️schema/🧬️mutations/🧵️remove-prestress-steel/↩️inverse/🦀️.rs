//! ↩️ `remove-prestress-steel` inverse: inserts the removed row back at its position.

use crate::remove_prestress_steel::RemovePrestressSteel;
use crate::insert_prestress_steel::InsertPrestressSteel;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &RemovePrestressSteel, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_prestress_steel::InsertPrestressSteel> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemovePrestressSteel, base: &En1992Snapshot) -> Option<InsertPrestressSteel> {

    let index = payload.index;
    let row = base.prestress_steels.get(index)?;
    Some(InsertPrestressSteel { index, prestress_steel: row.clone() })
}
