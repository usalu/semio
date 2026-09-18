//! ↩️ Inverse for `CreateRule` — the `delete-rule` of the id it created.

use crate::mutations::{delete_rule, Wfc3dMutation};
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn inverse(payload: &super::CreateRule, _base: &Wfc3dSnapshot) -> Vec<Wfc3dMutation> {
    vec![delete_rule(payload.rule.id.clone())]
}
