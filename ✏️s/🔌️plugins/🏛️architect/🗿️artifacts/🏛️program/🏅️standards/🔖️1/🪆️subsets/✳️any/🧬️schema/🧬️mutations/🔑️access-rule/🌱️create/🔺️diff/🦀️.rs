//! 🔺️ Sparse diff construction for the `create-access-rule` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔑access-rules` per Wave C.

use super::CreateAccessRule;
use crate::diff::ProgramAccessRulesDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateAccessRule, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.access_rule.header.id;
    if base.access_rules.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "An access rule already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.access_rules.len());
    if at > base.access_rules.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the access rule list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { access_rules: Some(ProgramAccessRulesDelta::insertion(at, payload.access_rule.clone())), ..Default::default() })
}
