//! 🔺️ Sparse diff builder for `DeleteTile` — removes the id from `tiles`, drops every rule naming
//! it, and releases every slot pinned to it (a replacement at each slot's OWN index).

use crate::diff::Wfc3dDiff;
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn diff(payload: &super::DeleteTile, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
    if !base.tiles.iter().any(|tile| tile.id == payload.id) {
        return protocol::MutationOutcome::error("wfc3d.tile.missing", format!("Tile \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let rules_removed: Vec<String> = base.rules.iter().filter(|rule| rule.tile_a_id == payload.id || rule.tile_b_id == payload.id).map(|rule| rule.id.clone()).collect();
    let slots_upserted: Vec<(usize, crate::schema::snapshot::Slot3d)> = base
        .slots
        .iter()
        .enumerate()
        .filter(|(_, slot)| slot.pinned_tile_id.as_deref() == Some(payload.id.as_str()))
        .map(|(index, slot)| {
            let mut released = slot.clone();
            released.pinned_tile_id = None;
            (index, released)
        })
        .collect();
    let cascaded_rules = rules_removed.len();
    let cascaded_pins = slots_upserted.len();
    let outcome = protocol::MutationOutcome::new(Wfc3dDiff { tiles_removed: vec![payload.id.clone()], rules_removed, slots_upserted, ..Default::default() });
    if cascaded_rules == 0 && cascaded_pins == 0 {
        outcome
    } else {
        outcome.info("wfc3d.tile.references-cascaded", format!("Deleting tile \"{}\" also removed {cascaded_rules} rule(s) and released {cascaded_pins} slot pin(s).", payload.id))
    }
}
