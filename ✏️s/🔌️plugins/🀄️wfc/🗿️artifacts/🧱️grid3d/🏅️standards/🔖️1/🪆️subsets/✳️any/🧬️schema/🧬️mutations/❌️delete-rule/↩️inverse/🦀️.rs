//! ↩️ Inverse for `DeleteRule` — the mutation list that carries the applied state back to
//! `base`, restoring row POSITION as well as row value.

use crate::standards::v1::subsets::any::schema::mutations::create_rule::create_rule;
use crate::schema::snapshot::*;

pub fn inverse<Op>(payload: &super::DeleteRule, base: &Grid3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::create_rule::CreateRule>,
{
    Ok((|| {
    let Some(rule) = base.rules.iter().find(|rule| rule.id == payload.id) else { return Vec::new() };
    vec![create_rule(rule.clone())]

    })())
}
