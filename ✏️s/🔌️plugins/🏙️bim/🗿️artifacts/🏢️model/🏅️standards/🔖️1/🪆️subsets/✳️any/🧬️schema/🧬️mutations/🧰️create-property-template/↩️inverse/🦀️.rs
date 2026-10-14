//! ↩️ Inverse of `CreatePropertyTemplate`: the concrete `DeletePropertyTemplate` of the id it created, none when the id was already taken.

use super::super::delete_property_template::DeletePropertyTemplate;
use super::super::elements;
use super::CreatePropertyTemplate;
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot};

pub fn inverse(payload: &CreatePropertyTemplate, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if elements::taken(base, &payload.id).is_some() {
        return Vec::new();
    }
    vec![ModelMutation::DeletePropertyTemplate(DeletePropertyTemplate { id: payload.id.clone() })]
}
