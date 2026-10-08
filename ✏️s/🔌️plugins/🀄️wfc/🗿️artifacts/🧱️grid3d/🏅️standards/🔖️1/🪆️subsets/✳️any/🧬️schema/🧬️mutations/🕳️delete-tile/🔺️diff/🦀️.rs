//! 🔺️ Sparse diff builder for `DeleteTile` — an id-keyed delta over `Grid3dSnapshot`, never a
//! whole-snapshot capture.

use crate::diff::{Grid3dDiff, Grid3dPinnedDelta, Grid3dRulesDelta, Grid3dTilesDelta};
use crate::schema::snapshot::*;

pub fn diff(payload: &super::DeleteTile, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
    if !base.tiles.iter().any(|tile| tile.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No tile with id \"{}\" exists.", payload.id), [payload.id.clone()]);
    }
    let rules_removed: Vec<String> = base.rules.iter().filter(|rule| rule.tile_a_id == payload.id || rule.tile_b_id == payload.id).map(|rule| rule.id.clone()).collect();
    let pinned_removed: Vec<String> = base.pinned.iter().filter(|cell| cell.tile_id == payload.id).map(|cell| cell_key(cell.x, cell.y, cell.z)).collect();
    protocol::MutationOutcome::new(Grid3dDiff { tiles: Grid3dTilesDelta::removal(&base.tiles, base.tiles.iter().position(|row| protocol::list_delta::Keyed::key(row) == payload.id.clone()).unwrap_or(usize::MAX)), rules: Grid3dRulesDelta::removals(&base.rules, &base.rules.iter().enumerate().filter(|(_, row)| rules_removed.contains(&protocol::list_delta::Keyed::key(*row))).map(|(index, _)| index).collect::<Vec<_>>()), pinned: Grid3dPinnedDelta::removals(&base.pinned, &base.pinned.iter().enumerate().filter(|(_, row)| pinned_removed.contains(&protocol::list_delta::Keyed::key(*row))).map(|(index, _)| index).collect::<Vec<_>>()), ..Default::default() })
}
