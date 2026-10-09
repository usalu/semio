//! ↩️ Inverse of `DeletePropertyTemplate`: the concrete `CreatePropertyTemplate` carrying the full removed record, none when the template was absent.

use super::super::create_property_template::CreatePropertyTemplate;
use super::DeletePropertyTemplate;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeletePropertyTemplate, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.property_templates.get(&payload.id) {
        Some(template) => vec![ModelMutation::CreatePropertyTemplate(CreatePropertyTemplate { id: payload.id.clone(), template: template.clone() })],
        None => Vec::new(),
    }
}
