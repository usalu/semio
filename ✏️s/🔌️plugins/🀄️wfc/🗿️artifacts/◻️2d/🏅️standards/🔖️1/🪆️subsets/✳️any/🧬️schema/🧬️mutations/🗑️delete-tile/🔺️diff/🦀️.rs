//! 🔺️ Sparse diff builder for `DeleteTile` — a real id-keyed delta, never a whole-snapshot capture.

use crate::diff::Wfc2dDiff;
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn diff(payload: &super::DeleteTile, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
    if !base.tiles.iter().any(|tile| tile.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Tile \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let orphaned_rules: Vec<String> = base.rules.iter().filter(|rule| rule.tile_a_id == payload.id || rule.tile_b_id == payload.id).map(|rule| rule.id.clone()).collect();
    let released: Vec<(usize, crate::schema::snapshot::Wfc2dSlot)> = base
        .slots
        .iter()
        .enumerate()
        .filter(|(_, slot)| slot.pinned_tile_id.as_deref() == Some(payload.id.as_str()))
        .map(|(index, slot)| (index, crate::schema::snapshot::Wfc2dSlot { pinned_tile_id: None, ..slot.clone() }))
        .collect();
    let cascaded = orphaned_rules.len() + released.len();
    let outcome = protocol::MutationOutcome::new(Wfc2dDiff { tiles_removed: vec![payload.id.clone()], rules_removed: orphaned_rules.clone(), slots_upserted: released.clone(), ..Default::default() });
    if cascaded == 0 {
        outcome
    } else {
        outcome.info("mutation.cascade", format!("Deleting tile \"{}\" also removed {} rule(s) and released {} pin(s).", payload.id, orphaned_rules.len(), released.len()))
    }
}
