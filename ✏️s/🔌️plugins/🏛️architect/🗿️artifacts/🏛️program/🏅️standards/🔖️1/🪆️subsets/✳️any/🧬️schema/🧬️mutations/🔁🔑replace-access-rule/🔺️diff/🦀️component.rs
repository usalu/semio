//! 🔺️ Sparse diff construction for the `replace-access-rule` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔑access-rules` per Wave C.

use super::mutation::ReplaceAccessRule;
use crate::artifacts::program::diff::{ProgramAccessRulesDelta, ProgramAccessRulesPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceAccessRule, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.access_rules.iter().find(|row| row.header.id == payload.access_rule.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No access rule exists with this id.", [payload.access_rule.header.id.0.clone()]);
    };
    if existing == &payload.access_rule {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This access rule already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.access_rule).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { access_rules: Some(ProgramAccessRulesDelta { patched: vec![ProgramAccessRulesPatchEntry { id: payload.access_rule.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
