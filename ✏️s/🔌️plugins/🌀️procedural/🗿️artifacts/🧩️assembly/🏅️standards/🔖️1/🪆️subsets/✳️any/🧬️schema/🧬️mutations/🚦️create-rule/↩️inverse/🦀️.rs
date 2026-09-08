//! ↩️ Inverse for `CreateRule` — the `delete-rule` of the id it created.

use crate::mutations::{delete_rule, AssemblyMutation};
use crate::schema::snapshot::AssemblySnapshot;

pub fn inverse(payload: &super::CreateRule, _base: &AssemblySnapshot) -> Vec<AssemblyMutation> {
    vec![delete_rule(payload.rule.id.clone())]
}
