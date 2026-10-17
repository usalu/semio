//! ↩️ Inverse for `DeleteRule` — built from a real BASE lookup, so a target the base never held
//! yields an empty inverse (nothing to undo) rather than a fabricated one.

use crate::standards::v1::subsets::any::schema::mutations::create_rule::create_rule;
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn inverse<Op>(payload: &super::DeleteRule, base: &Wfc2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::create_rule::CreateRule>,
{
    Ok((|| {
    let Some(rule) = base.rules.iter().find(|rule| rule.id == payload.id) else {
        return Vec::new();
    };
    vec![create_rule(rule.clone())]

    })())
}
