//! 🔺️ Sparse diff construction for the `create-access-rule` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔑access-rules` per Wave C.

use super::CreateAccessRule;
use crate::diff::ProgramAccessRulesDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.index-out-of-range` if `index` lies past the end (both empty diff); else `added = [payload row]`, plus `reordered` (the base order with the row inserted at `index`) unless the row lands last.
pub fn diff(payload: &CreateAccessRule, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.access_rule.header.id;
    if base.access_rules.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "An access rule already exists with this id.", [id.0.clone()]);
    }
    let length = base.access_rules.len();
    let at = payload.index.unwrap_or(length);
    if at > length {
        return protocol::MutationOutcome::error("mutation.index-out-of-range", "The index lies beyond the end of the access rule list.", [id.0.clone()]);
    }
    let reordered = (at < length).then(|| {
        let mut order: Vec<String> = base.access_rules.iter().map(|row| row.header.id.0.clone()).collect();
        order.insert(at, id.0.clone());
        order
    });
    protocol::MutationOutcome::new(ProgramDiff { access_rules: Some(ProgramAccessRulesDelta { added: vec![payload.access_rule.clone()], reordered, ..Default::default() }), ..Default::default() })
}
