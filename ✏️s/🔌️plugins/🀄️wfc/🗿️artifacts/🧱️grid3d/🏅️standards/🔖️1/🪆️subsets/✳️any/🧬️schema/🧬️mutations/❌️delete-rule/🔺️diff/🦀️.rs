//! 🔺️ Sparse diff builder for `DeleteRule` — an id-keyed delta over `Grid3dSnapshot`, never a
//! whole-snapshot capture.

use crate::diff::{Grid3dDiff, Grid3dRulesDelta};
use crate::schema::snapshot::*;

pub fn diff(payload: &super::DeleteRule, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
    if !base.rules.iter().any(|rule| rule.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No rule with id \"{}\" exists.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Grid3dDiff { rules: Grid3dRulesDelta::removal(&base.rules, base.rules.iter().position(|row| protocol::list_delta::Keyed::key(row) == payload.id.clone()).unwrap_or(usize::MAX)), ..Default::default() })
}
