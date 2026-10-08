//! ↩️ Inverse of `DeleteElements`: one concrete create per removed element (sites first in creation order, openings last) and one
//! property or classification setter per removed data entry, in storage order so that the store, which replays the vector reversed,
//! restores the base exactly. Empty when the selection is refused.

use super::super::cascade;
use super::DeleteElements;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteElements, base: &ModelSnapshot) -> Vec<ModelMutation> {
    cascade::inverse(base, &payload.ids)
}
