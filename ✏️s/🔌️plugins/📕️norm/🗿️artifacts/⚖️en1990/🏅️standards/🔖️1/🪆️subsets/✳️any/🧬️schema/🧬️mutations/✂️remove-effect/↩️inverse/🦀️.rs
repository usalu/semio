//! ✂️ `remove-effect` inverse — inserts the removed row back at its position; an absent row leaves nothing to restore.

use crate::remove_effect::RemoveEffect;
use crate::insert_effect::InsertEffect;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &RemoveEffect, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_effect::InsertEffect> {
    Ok(match previous(payload, base) {
        Some(inverse) => vec![Op::from(inverse)],
        None => Vec::new(),
    })
}

fn previous(payload: &RemoveEffect, base: &En1990Snapshot) -> Option<crate::insert_effect::InsertEffect> {
    base.effects.get(payload.index).map(|item| InsertEffect { index: Some(payload.index), item: item.clone() })
}
