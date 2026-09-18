//! ↩️ Inverse for `CreateRule` — built from a real BASE lookup, so a target the base never held
//! yields an empty inverse (nothing to undo) rather than a fabricated one.

use crate::mutations::{delete_rule, Wfc2dMutation};
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn inverse(payload: &super::CreateRule, _base: &Wfc2dSnapshot) -> Vec<Wfc2dMutation> {
    vec![delete_rule(payload.rule.id.clone())]
}
