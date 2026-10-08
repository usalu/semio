//! 📎 `insert-effect` inverse — removes the row at the position the insert landed on.

use super::InsertEffect;
use crate::mutations::remove_effect::RemoveEffect;
use crate::{En1990Mutation, En1990Snapshot};

pub fn inverse(payload: &InsertEffect, base: &En1990Snapshot) -> Result<Vec<En1990Mutation>, semio_framework_value::ValueError> {
    Ok(vec![En1990Mutation::RemoveEffect(RemoveEffect { index: payload.index.unwrap_or(usize::MAX).min(base.effects.len()) })])
}
