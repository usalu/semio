//! ↩️ Inverse for `DeleteTile` — recreates the tile, then every rule the delete cascaded away at its
//! own BASE position, then every pin it released — each a single step of this same vocabulary.

use crate::mutations::{create_rule, create_tile, pin_slot, Wfc3dMutation};
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn inverse(payload: &super::DeleteTile, base: &Wfc3dSnapshot) -> Vec<Wfc3dMutation> {
    let Some(index) = base.tiles.iter().position(|tile| tile.id == payload.id) else {
        return Vec::new();
    };
    let mut restore = vec![create_tile(index, base.tiles[index].clone())];
    for (rule_index, rule) in base.rules.iter().enumerate() {
        if rule.tile_a_id == payload.id || rule.tile_b_id == payload.id {
            restore.push(create_rule(rule_index, rule.clone()));
        }
    }
    for slot in &base.slots {
        if slot.pinned_tile_id.as_deref() == Some(payload.id.as_str()) {
            restore.push(pin_slot(slot.id.clone(), payload.id.clone()));
        }
    }
    restore
}
