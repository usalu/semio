//! 🔗 `change-effects` inverse — removes the collection's new rows, then inserts the base rows in order; the rows are stored in reverse, as the store replays inverses reversed.

use crate::change_effects::ChangeEffects;
use crate::insert_effect::InsertEffect;
use crate::remove_effect::RemoveEffect;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(mutation: &ChangeEffects, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_effect::RemoveEffect> + From<crate::insert_effect::InsertEffect> {
    let removed = (0..mutation.new_effects.len()).rev().map(|index| Op::from(RemoveEffect { index }));
    let inserted = base.effects.iter().cloned().enumerate().map(|(index, item)| Op::from(InsertEffect { index: Some(index), item }));
    Ok(removed.chain(inserted).rev().collect())
}
