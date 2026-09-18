//! ↩️ Inverse for `DeleteTile` — built from a real BASE lookup, so a target the base never held
//! yields an empty inverse (nothing to undo) rather than a fabricated one.

use crate::mutations::{create_rule, create_tile, pin_slot, Wfc2dMutation};
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn inverse(payload: &super::DeleteTile, base: &Wfc2dSnapshot) -> Vec<Wfc2dMutation> {
    let Some(tile) = base.tiles.iter().find(|tile| tile.id == payload.id) else {
        return Vec::new();
    };
    let mut restore = vec![create_tile(tile.clone())];
    for rule in &base.rules {
        if rule.tile_a_id == payload.id || rule.tile_b_id == payload.id {
            restore.push(create_rule(rule.clone()));
        }
    }
    for slot in &base.slots {
        if slot.pinned_tile_id.as_deref() == Some(payload.id.as_str()) {
            restore.push(pin_slot(slot.id.clone(), payload.id.clone()));
        }
    }
    restore
}
