//! 🔺️ Sparse diff builder for `DeleteRule` — one removal; the pair falls back to the FORBIDDEN
//! default for that direction.

use crate::diff::Grid2dDiff;
use crate::schema::snapshot::Grid2dSnapshot;

pub fn diff(payload: &super::DeleteRule, base: &Grid2dSnapshot) -> protocol::MutationOutcome<Grid2dDiff> {
    if !base.rules.iter().any(|rule| rule.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Rule \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Grid2dDiff { rules_removed: vec![payload.id.clone()], ..Default::default() })
}
