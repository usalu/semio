//! 🔺️ Sparse diff builder for `CreateRule` — a real id-keyed upsert into `rules`.

use crate::diff::Wfc3dDiff;
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn diff(payload: &super::CreateRule, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
    if base.rules.iter().any(|rule| rule.id == payload.rule.id) {
        return protocol::MutationOutcome::fatal("wfc3d.rule.duplicate-id", format!("A rule with id \"{}\" already exists.", payload.rule.id), [payload.rule.id.clone()]);
    }
    if !base.tiles.iter().any(|tile| tile.id == payload.rule.tile_a_id) {
        return protocol::MutationOutcome::fatal("wfc3d.rule.unknown-tile", format!("Rule \"{}\" references unknown tile \"{}\".", payload.rule.id, payload.rule.tile_a_id), [payload.rule.tile_a_id.clone()]);
    }
    if !base.tiles.iter().any(|tile| tile.id == payload.rule.tile_b_id) {
        return protocol::MutationOutcome::fatal("wfc3d.rule.unknown-tile", format!("Rule \"{}\" references unknown tile \"{}\".", payload.rule.id, payload.rule.tile_b_id), [payload.rule.tile_b_id.clone()]);
    }
    protocol::MutationOutcome::new(Wfc3dDiff { rules_upserted: vec![(payload.index, payload.rule.clone())], ..Default::default() })
}
