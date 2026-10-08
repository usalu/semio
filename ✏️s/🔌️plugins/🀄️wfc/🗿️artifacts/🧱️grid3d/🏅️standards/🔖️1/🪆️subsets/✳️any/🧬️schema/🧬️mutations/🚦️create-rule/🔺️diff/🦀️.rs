//! 🔺️ Sparse diff builder for `CreateRule` — an id-keyed delta over `Grid3dSnapshot`, never a
//! whole-snapshot capture.

use crate::diff::{Grid3dDiff, Grid3dRow, Grid3dRulesDelta};
use crate::schema::snapshot::*;

pub fn diff(payload: &super::CreateRule, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
    if payload.rule.id.is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A rule id cannot be empty.".to_string(), [String::new()]);
    }
    if base.rules.iter().any(|rule| rule.id == payload.rule.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A rule with id \"{}\" already exists.", payload.rule.id), [payload.rule.id.clone()]);
    }
    for tile in [&payload.rule.tile_a_id, &payload.rule.tile_b_id] {
        if !base.tiles.iter().any(|candidate| &candidate.id == tile) {
            return protocol::MutationOutcome::fatal("mutation.invariant", format!("Rule \"{}\" names unknown tile \"{tile}\".", payload.rule.id), [tile.clone()]);
        }
    }
    protocol::MutationOutcome::new(Grid3dDiff { rules: Grid3dRulesDelta::insertion(Grid3dRow::insert_at(&base.rules, &payload.rule), payload.rule.clone()), ..Default::default() })
}
