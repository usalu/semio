//! 🔺️ Sparse diff builder for `CreateRule` — a real id-keyed delta, never a whole-snapshot capture.

use crate::diff::Wfc2dDiff;
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn diff(payload: &super::CreateRule, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
    if base.rules.iter().any(|rule| rule.id == payload.rule.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A rule with id \"{}\" already exists.", payload.rule.id), [payload.rule.id.clone()]);
    }
    for tile_id in [&payload.rule.tile_a_id, &payload.rule.tile_b_id] {
        if !base.tiles.iter().any(|tile| &tile.id == tile_id) {
            return protocol::MutationOutcome::fatal("mutation.invariant", format!("Rule \"{}\" references unknown tile \"{}\".", payload.rule.id, tile_id), [tile_id.clone()]);
        }
    }
    let at = crate::mutations::ordered_index(&base.rules, &payload.rule.id, |rule| rule.id.as_str());
    protocol::MutationOutcome::new(Wfc2dDiff { rules_upserted: vec![(at, payload.rule.clone())], ..Default::default() })
}
