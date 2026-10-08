//! ➖ `remove-permanent` inverse — inserts the removed row back at its position; an absent row leaves nothing to restore.

use super::RemovePermanent;
use crate::mutations::insert_permanent::InsertPermanent;
use crate::{En1990Mutation, En1990Snapshot};

pub fn inverse(payload: &RemovePermanent, base: &En1990Snapshot) -> Result<Vec<En1990Mutation>, semio_framework_value::ValueError> {
    Ok(base.permanents.get(payload.index).map(|item| vec![En1990Mutation::InsertPermanent(InsertPermanent { index: payload.index, item: item.clone() })]).unwrap_or_default())
}
