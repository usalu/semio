//! ↩️ Inverse of `SetPropertyTemplate`: an absolute `SetPropertyTemplate` restoring the base value of exactly the fields the forward really changes, none when the template is absent or nothing changes.

use super::SetPropertyTemplate;
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot, diff::Patch};

pub fn inverse(payload: &SetPropertyTemplate, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.property_templates.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetPropertyTemplate(SetPropertyTemplate::from_patch(payload.id.clone(), restore))]
}
