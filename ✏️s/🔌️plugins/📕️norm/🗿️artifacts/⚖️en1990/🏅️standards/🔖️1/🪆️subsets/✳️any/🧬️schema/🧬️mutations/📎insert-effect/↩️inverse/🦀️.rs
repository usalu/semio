//! 📎 `insert-effect` inverse — removes the row at the position the insert landed on.

use crate::insert_effect::InsertEffect;
use crate::remove_effect::RemoveEffect;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &InsertEffect, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_effect::RemoveEffect> {
    Ok(vec![Op::from(RemoveEffect { index: payload.index.unwrap_or(usize::MAX).min(base.effects.len()) })])
}
