//! 🔺️ Sparse diff builder for `DeleteRule` — one removal; the pair falls back to the FORBIDDEN
//! default for that direction.

use crate::diff::{Grid2dDiff, Grid2dRulesDelta};
use crate::schema::snapshot::Grid2dSnapshot;

pub fn diff(payload: &super::DeleteRule, base: &Grid2dSnapshot) -> protocol::MutationOutcome<Grid2dDiff> {
    if !base.rules.iter().any(|rule| rule.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Rule \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Grid2dDiff { rules: Grid2dRulesDelta::removal(&base.rules, base.rules.iter().position(|row| protocol::list_delta::Keyed::key(row) == payload.id.clone()).unwrap_or(usize::MAX)), ..Default::default() })
}
