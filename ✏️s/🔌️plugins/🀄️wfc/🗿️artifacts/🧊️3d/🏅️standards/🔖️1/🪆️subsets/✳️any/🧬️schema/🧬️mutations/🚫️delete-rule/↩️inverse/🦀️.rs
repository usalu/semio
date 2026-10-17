//! ↩️ Inverse for `DeleteRule` — recreates the rule at its BASE position (missing id ⇒ empty).

use crate::standards::v1::subsets::any::schema::mutations::create_rule::create_rule;
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn inverse<Op>(payload: &super::DeleteRule, base: &Wfc3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::create_rule::CreateRule>,
{
    Ok((|| {
    let Some(index) = base.rules.iter().position(|rule| rule.id == payload.id) else {
        return Vec::new();
    };
    vec![create_rule(index, base.rules[index].clone())]

    })())
}
