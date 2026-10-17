//! ↩️ Inverse for `CreateRule` — the mutation list that carries the applied state back to
//! `base`, restoring row POSITION as well as row value.

use crate::standards::v1::subsets::any::schema::mutations::delete_rule::delete_rule;
use crate::schema::snapshot::*;

pub fn inverse<Op>(payload: &super::CreateRule, base: &Grid3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_rule::DeleteRule>,
{
    Ok((|| {
    let _ = base;
    vec![delete_rule(payload.rule.id.clone())]

    })())
}
