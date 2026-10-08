//! 🕳️ `remove-seismic` inverse — inserts the removed row back at its position; an absent row leaves nothing to restore.

use super::RemoveSeismic;
use crate::mutations::insert_seismic::InsertSeismic;
use crate::{En1990Mutation, En1990Snapshot};

pub fn inverse(payload: &RemoveSeismic, base: &En1990Snapshot) -> Result<Vec<En1990Mutation>, semio_framework_value::ValueError> {
    Ok(base.seismics.get(payload.index).map(|item| vec![En1990Mutation::InsertSeismic(InsertSeismic { index: payload.index, item: item.clone() })]).unwrap_or_default())
}
