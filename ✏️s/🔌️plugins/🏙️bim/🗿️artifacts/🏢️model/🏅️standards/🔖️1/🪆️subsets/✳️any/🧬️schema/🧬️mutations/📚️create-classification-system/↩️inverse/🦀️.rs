//! ↩️ Inverse of `CreateClassificationSystem`: the concrete `DeleteClassificationSystem` of the id it created, none when the id was already taken.

use super::super::delete_classification_system::DeleteClassificationSystem;
use super::super::elements;
use super::CreateClassificationSystem;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateClassificationSystem, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if elements::taken(base, &payload.id).is_some() || payload.id == "entry" {
        return Vec::new();
    }
    vec![ModelMutation::DeleteClassificationSystem(DeleteClassificationSystem { id: payload.id.clone() })]
}
