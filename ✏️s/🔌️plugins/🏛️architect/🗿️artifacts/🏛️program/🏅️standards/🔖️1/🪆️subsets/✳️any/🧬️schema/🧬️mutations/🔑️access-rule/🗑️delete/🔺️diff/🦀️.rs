//! 🔺️ Sparse diff construction for the `delete-access-rule` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔑access-rules` per Wave C.

use super::DeleteAccessRule;
use crate::diff::ProgramAccessRulesDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub fn diff(payload: &DeleteAccessRule, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.access_rules.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No access rule exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { access_rules: Some(ProgramAccessRulesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
