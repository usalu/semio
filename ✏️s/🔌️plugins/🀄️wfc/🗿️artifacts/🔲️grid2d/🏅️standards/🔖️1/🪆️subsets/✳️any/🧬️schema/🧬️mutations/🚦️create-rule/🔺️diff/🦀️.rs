//! 🔺️ Sparse diff builder for `CreateRule` — a real id-keyed upsert at the canonical sorted
//! position, refusing a dangling tile reference or a second rule on the same `(A, B, direction)`.

use crate::diff::Grid2dDiff;
use crate::mutations::ordered_rule_index;
use crate::schema::snapshot::Grid2dSnapshot;

pub fn diff(payload: &super::CreateRule, base: &Grid2dSnapshot) -> protocol::MutationOutcome<Grid2dDiff> {
    let rule = &payload.rule;
    if rule.id.is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A rule id may not be empty.".to_string(), ["rule".to_string()]);
    }
    if base.rules.iter().any(|existing| existing.id == rule.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A rule with id \"{}\" already exists.", rule.id), [rule.id.clone()]);
    }
    for tile_id in [&rule.tile_a_id, &rule.tile_b_id] {
        if !base.tiles.iter().any(|tile| &tile.id == tile_id) {
            return protocol::MutationOutcome::fatal("mutation.invariant", format!("Rule \"{}\" references unknown tile \"{tile_id}\".", rule.id), [tile_id.clone()]);
        }
    }
    if let Some(existing) = base.rules.iter().find(|existing| existing.tile_a_id == rule.tile_a_id && existing.tile_b_id == rule.tile_b_id && existing.direction == rule.direction) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Rule \"{}\" already constrains this pair and direction.", existing.id), [existing.id.clone()]);
    }
    let at = ordered_rule_index(&base.rules, &rule.id);
    protocol::MutationOutcome::new(Grid2dDiff { rules_upserted: vec![(at, rule.clone())], ..Default::default() })
}
