//! 🔺️ Sparse diff builder for `DeleteRule` — an id-keyed delta over `Grid3dSnapshot`, never a
//! whole-snapshot capture.

use crate::diff::Grid3dDiff;
use crate::schema::snapshot::*;

pub fn diff(payload: &super::DeleteRule, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
    if !base.rules.iter().any(|rule| rule.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.missing-target", format!("No rule with id \"{}\" exists.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Grid3dDiff { rules_removed: vec![payload.id.clone()], ..Default::default() })
}
