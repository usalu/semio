//! 🔺️ Sparse diff builder for `DeleteTile` — removes the id from `tiles`, drops every rule naming
//! it, and releases every slot pinned to it (a replacement at each slot's OWN index).

use crate::diff::{Wfc3dDiff, Wfc3dRowPatch, Wfc3dRows, Wfc3dOptionalText, Wfc3dSlotPatch};
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn diff(payload: &super::DeleteTile, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
    if !base.tiles.iter().any(|tile| tile.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Tile \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let rules_removed: Vec<String> = base.rules.iter().filter(|rule| rule.tile_a_id == payload.id || rule.tile_b_id == payload.id).map(|rule| rule.id.clone()).collect();
    let released: Vec<Wfc3dRowPatch<Wfc3dSlotPatch>> = base
        .slots
        .iter()
        .filter(|slot| slot.pinned_tile_id.as_deref() == Some(payload.id.as_str()))
        .map(|slot| Wfc3dRowPatch { id: slot.id.clone(), patch: Wfc3dSlotPatch { pinned_tile_id: Some(Wfc3dOptionalText { value: None }), ..Default::default() } })
        .collect();
    let cascaded_rules = rules_removed.len();
    let cascaded_pins = released.len();
    let outcome = protocol::MutationOutcome::new(Wfc3dDiff { tiles: Wfc3dRows { removed: vec![payload.id.clone()], ..Default::default() }, rules: Wfc3dRows { removed: rules_removed, ..Default::default() }, slots: Wfc3dRows { patched: released.clone(), ..Default::default() }, ..Default::default() });
    if cascaded_rules == 0 && cascaded_pins == 0 {
        outcome
    } else {
        outcome.info("mutation.cascade", format!("Deleting tile \"{}\" also removed {cascaded_rules} rule(s) and released {cascaded_pins} slot pin(s).", payload.id))
    }
}
