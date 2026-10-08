//! 🌋 `insert-seismic` inverse — removes the row at the position the insert landed on.

use super::InsertSeismic;
use crate::mutations::remove_seismic::RemoveSeismic;
use crate::{En1990Mutation, En1990Snapshot};

pub fn inverse(payload: &InsertSeismic, base: &En1990Snapshot) -> Result<Vec<En1990Mutation>, semio_framework_value::ValueError> {
    Ok(vec![En1990Mutation::RemoveSeismic(RemoveSeismic { index: payload.index.unwrap_or(usize::MAX).min(base.seismics.len()) })])
}
