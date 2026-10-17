//! ↩️ Inverse for `DeleteTile` — built from a real BASE lookup, so a target the base never held
//! yields an empty inverse (nothing to undo) rather than a fabricated one.

use crate::standards::v1::subsets::any::schema::mutations::create_rule::create_rule;
use crate::standards::v1::subsets::any::schema::mutations::create_tile::create_tile;
use crate::standards::v1::subsets::any::schema::mutations::pin_slot::pin_slot;
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn inverse<Op>(payload: &super::DeleteTile, base: &Wfc2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::create_rule::CreateRule> + From<crate::standards::v1::subsets::any::schema::mutations::create_tile::CreateTile> + From<crate::standards::v1::subsets::any::schema::mutations::pin_slot::PinSlot>,
{
    Ok((|| {
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
    restore.reverse();
    restore

    })())
}
