//! ↩️ Inverse of `CreateAreaScheme`: the concrete `DeleteAreaScheme` of the id it created, none when the id was already taken.

use super::super::delete_area_scheme::DeleteAreaScheme;
use super::super::elements;
use super::CreateAreaScheme;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateAreaScheme, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if elements::taken(base, &payload.id).is_some() {
        return Vec::new();
    }
    vec![ModelMutation::DeleteAreaScheme(DeleteAreaScheme { id: payload.id.clone() })]
}
