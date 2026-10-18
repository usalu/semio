//! ↩️ Inverse of `SetPropertyTemplate`: an absolute `SetPropertyTemplate` restoring the base value of exactly the fields the forward really changes, none when the template is absent or nothing changes.

use crate::standards::v1::subsets::any::schema::mutations::set_property_template::SetPropertyTemplate;
use crate::standards::v1::subsets::any::schema::{snapshot::ModelSnapshot, diff::Patch};

pub fn inverse<Op>(payload: &SetPropertyTemplate, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_property_template::SetPropertyTemplate>,
{
    let Some(record) = base.property_templates.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![Op::from(SetPropertyTemplate::from_patch(payload.id.clone(), restore))]
}
