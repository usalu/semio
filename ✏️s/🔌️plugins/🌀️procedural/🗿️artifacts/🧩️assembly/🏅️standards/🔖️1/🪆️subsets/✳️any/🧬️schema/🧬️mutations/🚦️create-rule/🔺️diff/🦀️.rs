//! 🔺️ Sparse diff builder for `CreateRule` — a real id-keyed upsert into `rules`.

use crate::artifacts::assembly::diff::AssemblyDiff;
use crate::artifacts::assembly::schema::snapshot::AssemblySnapshot;

pub fn diff(payload: &super::CreateRule, base: &AssemblySnapshot) -> protocol::MutationOutcome<AssemblyDiff> {
    if base.rules.iter().any(|rule| rule.id == payload.rule.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A rule with id \"{}\" already exists.", payload.rule.id), [payload.rule.id.clone()]);
    }
    if !base.modules.iter().any(|module| module.child_id == payload.rule.module_a_id) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Rule \"{}\" references unknown module \"{}\".", payload.rule.id, payload.rule.module_a_id), [payload.rule.module_a_id.clone()]);
    }
    if !base.modules.iter().any(|module| module.child_id == payload.rule.module_b_id) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Rule \"{}\" references unknown module \"{}\".", payload.rule.id, payload.rule.module_b_id), [payload.rule.module_b_id.clone()]);
    }
    protocol::MutationOutcome::new(AssemblyDiff { rules_upserted: vec![(payload.index, payload.rule.clone())], ..Default::default() })
}
