//! ↩️ Inverse of `DeletePropertyTemplate`: the concrete `CreatePropertyTemplate` carrying the full removed record, none when the template was absent.

use crate::standards::v1::subsets::any::schema::mutations::create_property_template::CreatePropertyTemplate;
use crate::standards::v1::subsets::any::schema::mutations::delete_property_template::DeletePropertyTemplate;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &DeletePropertyTemplate, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::create_property_template::CreatePropertyTemplate>,
{
    match base.property_templates.get(&payload.id) {
        Some(template) => vec![Op::from(CreatePropertyTemplate { id: payload.id.clone(), template: template.clone() })],
        None => Vec::new(),
    }
}
