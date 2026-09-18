//! ↩️ Inverse for `CreateRule` — the mutation list that carries the applied state back to
//! `base`, restoring row POSITION as well as row value.

use crate::mutations::Grid3dMutation;
use crate::schema::snapshot::*;

pub fn inverse(payload: &super::CreateRule, base: &Grid3dSnapshot) -> Vec<Grid3dMutation> {
    let _ = base;
    vec![crate::mutations::delete_rule(payload.rule.id.clone())]
}
