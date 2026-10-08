//! ✂️ `remove-effect` inverse — inserts the removed row back at its position; an absent row leaves nothing to restore.

use super::RemoveEffect;
use crate::mutations::insert_effect::InsertEffect;
use crate::{En1990Mutation, En1990Snapshot};

pub fn inverse(payload: &RemoveEffect, base: &En1990Snapshot) -> Result<Vec<En1990Mutation>, semio_framework_value::ValueError> {
    Ok(base.effects.get(payload.index).map(|item| vec![En1990Mutation::InsertEffect(InsertEffect { index: Some(payload.index), item: item.clone() })]).unwrap_or_default())
}
