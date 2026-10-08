//! ➕ `insert-permanent` inverse — removes the row at the position the insert landed on.

use super::InsertPermanent;
use crate::mutations::remove_permanent::RemovePermanent;
use crate::{En1990Mutation, En1990Snapshot};

pub fn inverse(payload: &InsertPermanent, base: &En1990Snapshot) -> Result<Vec<En1990Mutation>, semio_framework_value::ValueError> {
    Ok(vec![En1990Mutation::RemovePermanent(RemovePermanent { index: payload.index.min(base.permanents.len()) })])
}
