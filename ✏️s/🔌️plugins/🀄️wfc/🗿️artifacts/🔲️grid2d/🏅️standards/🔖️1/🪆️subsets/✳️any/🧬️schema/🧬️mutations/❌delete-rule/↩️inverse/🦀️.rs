//! ↩️ Inverse for `DeleteRule` — recreates the exact row from a real BASE lookup.

use crate::standards::v1::subsets::any::schema::mutations::create_rule::create_rule;
use crate::schema::snapshot::Grid2dSnapshot;

pub fn inverse<Op>(payload: &super::DeleteRule, base: &Grid2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::create_rule::CreateRule>,
{
    Ok((|| {
    match base.rules.iter().find(|rule| rule.id == payload.id) {
        Some(rule) => vec![create_rule(rule.clone())],
        None => Vec::new(),
    }

    })())
}
