//! 🔺️ Sparse diff builder for `CreateRule` — an id-keyed delta over `Grid3dSnapshot`, never a
//! whole-snapshot capture.

use crate::diff::Grid3dDiff;
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
    let at = crate::mutations::ordered_index(&base.rules, &payload.rule.id, |rule| rule.id.clone());
    protocol::MutationOutcome::new(Grid3dDiff { rules_upserted: vec![(at, payload.rule.clone())], ..Default::default() })
}
