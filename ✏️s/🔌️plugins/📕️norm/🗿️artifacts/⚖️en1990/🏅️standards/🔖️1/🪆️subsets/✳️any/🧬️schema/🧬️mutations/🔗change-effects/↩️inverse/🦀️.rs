//! 🔗 `change-effects` inverse — removes the collection's new rows, then inserts the base rows in order; the rows are stored in reverse, as the store replays inverses reversed.

use super::ChangeEffects;
use crate::mutations::insert_effect::InsertEffect;
use crate::mutations::remove_effect::RemoveEffect;
use crate::{En1990Mutation, En1990Snapshot};

pub fn inverse(mutation: &ChangeEffects, base: &En1990Snapshot) -> Result<Vec<En1990Mutation>, semio_framework_value::ValueError> {
    let removed = (0..mutation.new_effects.len()).rev().map(|index| En1990Mutation::RemoveEffect(RemoveEffect { index }));
    let inserted = base.effects.iter().cloned().enumerate().map(|(index, item)| En1990Mutation::InsertEffect(InsertEffect { index, item }));
    Ok(removed.chain(inserted).rev().collect())
}
