//! ↩️ Inverse for `CreateRule` — deletes the id it created.

use crate::standards::v1::subsets::any::schema::mutations::delete_rule::delete_rule;
use crate::schema::snapshot::Grid2dSnapshot;

pub fn inverse<Op>(payload: &super::CreateRule, base: &Grid2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_rule::DeleteRule>,
{
    Ok((|| {
    if base.rules.iter().any(|rule| rule.id == payload.rule.id) {
        return Vec::new();
    }
    vec![delete_rule(payload.rule.id.clone())]

    })())
}
