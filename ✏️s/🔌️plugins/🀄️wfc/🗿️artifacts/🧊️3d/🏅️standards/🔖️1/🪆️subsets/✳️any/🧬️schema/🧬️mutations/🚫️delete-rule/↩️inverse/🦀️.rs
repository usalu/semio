//! ↩️ Inverse for `DeleteRule` — recreates the rule at its BASE position (missing id ⇒ empty).

use crate::mutations::{create_rule, Wfc3dMutation};
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn inverse(payload: &super::DeleteRule, base: &Wfc3dSnapshot) -> Result<Vec<Wfc3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(index) = base.rules.iter().position(|rule| rule.id == payload.id) else {
        return Vec::new();
    };
    vec![create_rule(index, base.rules[index].clone())]

    })())
}
