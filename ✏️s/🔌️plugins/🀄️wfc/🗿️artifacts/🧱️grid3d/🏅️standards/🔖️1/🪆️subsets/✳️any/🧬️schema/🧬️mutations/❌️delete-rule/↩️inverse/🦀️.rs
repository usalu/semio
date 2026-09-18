//! ↩️ Inverse for `DeleteRule` — the mutation list that carries the applied state back to
//! `base`, restoring row POSITION as well as row value.

use crate::mutations::Grid3dMutation;
use crate::schema::snapshot::*;

pub fn inverse(payload: &super::DeleteRule, base: &Grid3dSnapshot) -> Vec<Grid3dMutation> {
    let Some(rule) = base.rules.iter().find(|rule| rule.id == payload.id) else { return Vec::new() };
    vec![crate::mutations::create_rule(rule.clone())]
}
