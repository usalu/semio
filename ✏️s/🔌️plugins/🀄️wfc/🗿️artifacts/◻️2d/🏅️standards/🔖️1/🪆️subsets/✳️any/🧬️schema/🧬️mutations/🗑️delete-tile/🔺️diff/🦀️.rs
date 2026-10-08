//! 🔺️ Sparse diff builder for `DeleteTile` — a real id-keyed delta, never a whole-snapshot capture.

use crate::diff::{Wfc2dDiff, Wfc2dRowPatch, Wfc2dRows, Wfc2dOptionalText, Wfc2dSlotPatch};
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn diff(payload: &super::DeleteTile, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
    if !base.tiles.iter().any(|tile| tile.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Tile \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let orphaned_rules: Vec<String> = base.rules.iter().filter(|rule| rule.tile_a_id == payload.id || rule.tile_b_id == payload.id).map(|rule| rule.id.clone()).collect();
    let released: Vec<Wfc2dRowPatch<Wfc2dSlotPatch>> = base
        .slots
        .iter()
        .filter(|slot| slot.pinned_tile_id.as_deref() == Some(payload.id.as_str()))
        .map(|slot| Wfc2dRowPatch { id: slot.id.clone(), patch: Wfc2dSlotPatch { pinned_tile_id: Some(Wfc2dOptionalText { value: None }), ..Default::default() } })
        .collect();
    let cascaded = orphaned_rules.len() + released.len();
    let outcome = protocol::MutationOutcome::new(Wfc2dDiff { tiles: Wfc2dRows { removed: vec![payload.id.clone()], ..Default::default() }, rules: Wfc2dRows { removed: orphaned_rules.clone(), ..Default::default() }, slots: Wfc2dRows { patched: released.clone(), ..Default::default() }, ..Default::default() });
    if cascaded == 0 {
        outcome
    } else {
        outcome.info("mutation.cascade", format!("Deleting tile \"{}\" also removed {} rule(s) and released {} pin(s).", payload.id, orphaned_rules.len(), released.len()))
    }
}
