//! 🔺️ Sparse diff construction for the `delete-access-rule` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔑access-rules` per Wave C.

use super::DeleteAccessRule;
use crate::standards::v1::subsets::any::schema::diff::ProgramAccessRulesDelta;
use crate::standards::v1::subsets::any::schema::diff::ProgramDiff;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [{id, index}]`.
pub fn diff(payload: &DeleteAccessRule, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(position) = base.access_rules.iter().position(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No access rule exists with this id.", [payload.id.0.clone()]);
    };
    protocol::MutationOutcome::new(ProgramDiff { access_rules: Some(ProgramAccessRulesDelta::removal(&base.access_rules, position)), ..Default::default() })
}
