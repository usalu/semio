//! ↩️ Inverse of `CreatePropertyTemplate`: the concrete `DeletePropertyTemplate` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_property_template::DeletePropertyTemplate;
use crate::standards::v1::subsets::any::schema::mutations::elements;
use crate::standards::v1::subsets::any::schema::mutations::create_property_template::CreatePropertyTemplate;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreatePropertyTemplate, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_property_template::DeletePropertyTemplate>,
{
    if elements::taken(base, &payload.id).is_some() {
        return Vec::new();
    }
    vec![Op::from(DeletePropertyTemplate { id: payload.id.clone() })]
}
