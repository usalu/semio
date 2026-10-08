//! 🔺️ Sparse diff construction for the `replace-access-rule` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔑access-rules` per Wave C.

use super::ReplaceAccessRule;
use crate::diff::ProgramAccessRulesDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns:
/// `removed = [id]`, `added = [payload row]`, and `reordered` (the base order) unless the row was last, so the new row keeps its position.
pub fn diff(payload: &ReplaceAccessRule, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.access_rule.header.id;
    let Some(position) = base.access_rules.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No access rule exists with this id.", [id.0.clone()]);
    };
    if base.access_rules[position] == payload.access_rule {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This access rule already matches the requested value.").at([id.0.clone()])]);
    }
    let reordered = (position + 1 != base.access_rules.len()).then(|| base.access_rules.iter().map(|row| row.header.id.0.clone()).collect());
    protocol::MutationOutcome::new(ProgramDiff { access_rules: Some(ProgramAccessRulesDelta { removed: vec![id.0.clone()], added: vec![payload.access_rule.clone()], reordered, ..Default::default() }), ..Default::default() })
}
