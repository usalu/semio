//! 🔺️ Sparse diff builder for `CreateRule` — a real id-keyed upsert into `rules`.

use crate::diff::{Wfc3dDiff, Wfc3dRows};
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn diff(payload: &super::CreateRule, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
    if base.rules.iter().any(|rule| rule.id == payload.rule.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A rule with id \"{}\" already exists.", payload.rule.id), [payload.rule.id.clone()]);
    }
    if !base.tiles.iter().any(|tile| tile.id == payload.rule.tile_a_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Rule \"{}\" references unknown tile \"{}\".", payload.rule.id, payload.rule.tile_a_id), [payload.rule.tile_a_id.clone()]);
    }
    if !base.tiles.iter().any(|tile| tile.id == payload.rule.tile_b_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Rule \"{}\" references unknown tile \"{}\".", payload.rule.id, payload.rule.tile_b_id), [payload.rule.tile_b_id.clone()]);
    }
    let canonical = crate::schema::snapshot::canonical_rule_index(base, &payload.rule.id);
    if payload.index != canonical {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Rule \"{}\" must be inserted at its canonical position {canonical}, not {}.", payload.rule.id, payload.index), [payload.rule.id.clone()]);
    }
    protocol::MutationOutcome::new(Wfc3dDiff { rules: Wfc3dRows { added: vec![payload.rule.clone()], ..Default::default() }, ..Default::default() })
}
