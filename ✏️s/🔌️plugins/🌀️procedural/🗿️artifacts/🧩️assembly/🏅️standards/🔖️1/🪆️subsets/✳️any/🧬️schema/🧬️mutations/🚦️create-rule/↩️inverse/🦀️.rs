//! ↩️ Inverse for `CreateRule` — the `delete-rule` of the id it created.

use crate::artifacts::assembly::mutations::{delete_rule, AssemblyMutation};
use crate::artifacts::assembly::schema::snapshot::AssemblySnapshot;

pub fn inverse(payload: &super::CreateRule, _base: &AssemblySnapshot) -> Vec<AssemblyMutation> {
    vec![delete_rule(payload.rule.id.clone())]
}
