//! ↩️ Inverse for `CreateRule` — built from a real BASE lookup, so a target the base never held
//! yields an empty inverse (nothing to undo) rather than a fabricated one.

use crate::standards::v1::subsets::any::schema::mutations::delete_rule::delete_rule;
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn inverse<Op>(payload: &super::CreateRule, _base: &Wfc2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_rule::DeleteRule>,
{
    Ok((|| {
    vec![delete_rule(payload.rule.id.clone())]

    })())
}
