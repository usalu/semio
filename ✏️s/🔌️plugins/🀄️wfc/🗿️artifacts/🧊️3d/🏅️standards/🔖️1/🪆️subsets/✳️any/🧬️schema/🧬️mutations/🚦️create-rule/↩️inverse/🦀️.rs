//! ↩️ Inverse for `CreateRule` — the `delete-rule` of the id it created.

use crate::standards::v1::subsets::any::schema::mutations::delete_rule::delete_rule;
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn inverse<Op>(payload: &super::CreateRule, _base: &Wfc3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_rule::DeleteRule>,
{
    Ok((|| {
    vec![delete_rule(payload.rule.id.clone())]

    })())
}
