//! ↩️ Inverse for `DeleteRule` — built from a real BASE lookup, so a target the base never held
//! yields an empty inverse (nothing to undo) rather than a fabricated one.

use crate::mutations::{create_rule, Wfc2dMutation};
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn inverse(payload: &super::DeleteRule, base: &Wfc2dSnapshot) -> Vec<Wfc2dMutation> {
    let Some(rule) = base.rules.iter().find(|rule| rule.id == payload.id) else {
        return Vec::new();
    };
    vec![create_rule(rule.clone())]
}
