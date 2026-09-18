//! 🔺️ Sparse diff builder for `DeleteTile` — an id-keyed delta over `Grid3dSnapshot`, never a
//! whole-snapshot capture.

use crate::diff::Grid3dDiff;
use crate::schema::snapshot::*;

pub fn diff(payload: &super::DeleteTile, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
    if !base.tiles.iter().any(|tile| tile.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.missing-target", format!("No tile with id \"{}\" exists.", payload.id), [payload.id.clone()]);
    }
    let rules_removed: Vec<String> = base.rules.iter().filter(|rule| rule.tile_a_id == payload.id || rule.tile_b_id == payload.id).map(|rule| rule.id.clone()).collect();
    let pinned_removed: Vec<String> = base.pinned.iter().filter(|cell| cell.tile_id == payload.id).map(|cell| cell_key(cell.x, cell.y, cell.z)).collect();
    protocol::MutationOutcome::new(Grid3dDiff { tiles_removed: vec![payload.id.clone()], rules_removed, pinned_removed, ..Default::default() })
}
