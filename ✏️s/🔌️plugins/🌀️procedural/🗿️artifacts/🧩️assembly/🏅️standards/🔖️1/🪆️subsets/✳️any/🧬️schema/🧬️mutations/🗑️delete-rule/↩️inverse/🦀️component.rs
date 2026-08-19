//! ↩️ Inverse for `DeleteRule` — recreates the rule from a real BASE lookup (missing id ⇒ empty).

use crate::artifacts::assembly::mutations::{create_rule, AssemblyMutation};
use crate::artifacts::assembly::schema::snapshot::AssemblySnapshot;

pub async fn inverse(payload: &super::mutation::DeleteRule, base: &AssemblySnapshot) -> Vec<AssemblyMutation> {
    let Some(index) = base.rules.iter().position(|rule| rule.id == payload.id) else {
        return Vec::new();
    };
    vec![create_rule(index, base.rules[index].clone())]
}
