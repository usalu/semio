//! ↩️ Inverse of `ArrayElements`: one `DeleteElements` of the minted ids of every copied element in every copy; the cascade removes their
//! openings and their property and classification entries, which is exactly what the array created. Empty when the array is refused.

use super::super::delete_elements::DeleteElements;
use super::super::modify;
use super::diff::maps_of;
use super::ArrayElements;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &ArrayElements, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(maps) = maps_of(payload) else {
        return Vec::new();
    };
    match modify::duplicate(base, &payload.ids, &payload.prefix, &maps) {
        Ok(built) if !built.roots.is_empty() => vec![ModelMutation::DeleteElements(DeleteElements { ids: built.roots })],
        _ => Vec::new(),
    }
}
