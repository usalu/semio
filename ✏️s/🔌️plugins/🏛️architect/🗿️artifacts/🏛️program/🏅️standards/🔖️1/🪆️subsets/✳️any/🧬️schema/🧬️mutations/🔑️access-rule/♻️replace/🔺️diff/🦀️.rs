//! 🔺️ Sparse diff construction for the `replace-access-rule` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔑access-rules` per Wave C.

use super::ReplaceAccessRule;
use crate::diff::ProgramAccessRulesDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceAccessRule, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.access_rule.header.id;
    let Some(position) = base.access_rules.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No access rule exists with this id.", [id.0.clone()]);
    };
    if base.access_rules[position] == payload.access_rule {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This access rule already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramAccessRulesDelta::removal(&base.access_rules, position);
    delta.absorb(ProgramAccessRulesDelta::insertion(position, payload.access_rule.clone()));
    protocol::MutationOutcome::new(ProgramDiff { access_rules: Some(delta), ..Default::default() })
}
